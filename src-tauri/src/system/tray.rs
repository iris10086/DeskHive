use tauri::{menu::{MenuBuilder, MenuItemBuilder}, tray::{TrayIconBuilder, TrayIconEvent}};

use crate::data::{load_app_settings, save_app_settings};
use crate::window::management::{open_settings_window, show_main_window};
use crate::quit_app;

use std::sync::Mutex;

/// 存储"鼠标穿透"菜单项更新回调，供 `save_app_settings` 等模块直接调用，
/// 实现 Setting 页面、托盘菜单、实际效果三者的实时双向同步。
type TrayTextUpdater = Box<dyn Fn(bool) + Send + Sync>;
static CLICK_THROUGH_UPDATER: Mutex<Option<TrayTextUpdater>> = Mutex::new(None);

/// 由 `save_app_settings` 调用，同步更新托盘菜单中"鼠标穿透"项的勾选文本。
pub fn set_click_through_tray_text(enabled: bool) {
    if let Some(ref updater) = *CLICK_THROUGH_UPDATER.lock().unwrap() {
        updater(enabled);
    }
}

// 创建系统托盘菜单和事件处理
pub fn create_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 创建系统托盘菜单
    let show = MenuItemBuilder::with_id("show", "显示").build(app)?;
    let reset_position = MenuItemBuilder::with_id("reset_position", "重置窗口位置").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let click_through = MenuItemBuilder::with_id("click_through", "鼠标穿透").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&show, &reset_position, &settings, &click_through, &quit])
        .build()?;

    // 克隆 click_through 菜单项，用于两个场景：
    // 1. click_through_item — 托盘菜单点击事件处理
    // 2. click_through_updater — 注册到静态回调，供保存设置时调用
    let click_through_item = click_through.clone();
    let click_through_for_updater = click_through.clone();

    // 注册全局更新回调：Setting 页面保存时通过此回调实时更新托盘菜单文本
    // （托盘菜单点击切换后也会走到 save_app_settings → 此回调，文本会更新两次但幂等安全）
    *CLICK_THROUGH_UPDATER.lock().unwrap() = Some(Box::new(move |enabled: bool| {
        let _ = click_through_for_updater.set_text(if enabled { "✓ 鼠标穿透" } else { "鼠标穿透" });
    }));

    // 初始化托盘菜单文本：从已保存的设置中读取 click_through 状态
    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        if let Ok(settings) = load_app_settings(handle).await {
            set_click_through_tray_text(settings.click_through);
        }
    });

    // 创建系统托盘图标
    let _tray = TrayIconBuilder::with_id("main")
        .menu(&menu)
        .tooltip("Todo 桌面助手")
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } => {
                    // 左键单击只显示窗口
                    let app = tray.app_handle().clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = show_main_window(app).await;
                    });
                }
                _ => {}
            }
        })
        .on_menu_event(move |app, event| {
            match event.id().as_ref() {
                "show" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = show_main_window(app_handle).await;
                    });
                }
                "reset_position" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        use crate::window::management::reset_window_position;
                        let _ = reset_window_position(app_handle).await;
                    });
                }
                "settings" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = open_settings_window(app_handle).await;
                    });
                }
                "click_through" => {
                    let app_handle = app.clone();
                    let item = click_through_item.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Ok(mut settings) = load_app_settings(app_handle.clone()).await {
                            settings.click_through = !settings.click_through;
                            let enabled = settings.click_through;
                            let _ = save_app_settings(app_handle, settings).await;
                            // save_app_settings 内部会调用 set_click_through_tray_text，
                            // 此处显式更新以确保即时性（幂等安全）
                            let _ = item.set_text(if enabled { "✓ 鼠标穿透" } else { "鼠标穿透" });
                        }
                    });
                }
                "quit" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = quit_app(app_handle).await;
                    });
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
