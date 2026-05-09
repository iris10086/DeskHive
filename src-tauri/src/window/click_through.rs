#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowLongPtrW, GetWindowLongPtrW, SetWindowPos,
    GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT,
    SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE, SWP_NOZORDER, SWP_FRAMECHANGED,
};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

// 设置窗口鼠标穿透
#[cfg(target_os = "windows")]
pub fn set_click_through(window: &tauri::WebviewWindow, enabled: bool) -> Result<(), String> {
    let window_handle = window.window_handle()
        .map_err(|e| format!("获取窗口句柄失败: {}", e))?;

    let raw_handle = window_handle.as_raw();

    if let RawWindowHandle::Win32(handle) = raw_handle {
        unsafe {
            let hwnd = HWND(handle.hwnd.get() as isize);

            // 获取当前窗口扩展样式
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);

            if enabled {
                // 添加穿透样式（保留已有的 WS_EX_LAYERED 等样式）
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_TRANSPARENT.0 as isize | WS_EX_LAYERED.0 as isize);
            } else {
                // 移除穿透样式（保留 WS_EX_LAYERED 等其他样式）
                let new_style = (ex_style as u32 & !WS_EX_TRANSPARENT.0) | WS_EX_LAYERED.0;
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style as isize);
            }

            // 必须调用 SetWindowPos 并带上 SWP_FRAMECHANGED 标志，
            // 否则 SetWindowLongPtrW 的样式修改不会立即生效
            SetWindowPos(
                hwnd,
                HWND::default(),
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOZORDER | SWP_FRAMECHANGED,
            );
        }
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn set_click_through(_window: &tauri::WebviewWindow, _enabled: bool) -> Result<(), String> {
    // 在非Windows平台上，暂时不支持鼠标穿透设置
    Ok(())
}
