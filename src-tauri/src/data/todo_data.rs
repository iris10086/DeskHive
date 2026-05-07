use std::fs;
use std::path::PathBuf;
use serde_json;
use tauri::Manager;

use crate::models::{Todo, TodoData};

// 获取数据目录路径
fn get_data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    // 使用文档目录而不是应用数据目录，这样重装或更新应用时数据不会丢失
    let document_dir = app.path().document_dir()
        .map_err(|e| format!("获取用户文档目录失败: {}", e))?;
    
    // 创建DeskHive专用的数据目录
    let data_dir = document_dir.join("DeskHive").join("data");
    
    // 确保data目录存在
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("创建data目录失败: {}", e))?;
    }
    
    Ok(data_dir)
}

// Tauri 命令：保存todo数据
#[tauri::command]
pub async fn save_todo_data(app: tauri::AppHandle, pending_todos: Vec<Todo>, completed_todos: Vec<Todo>) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("todo_list.json");
    
    log::info!("保存待办事项数据: 待完成={}, 已完成={}", pending_todos.len(), completed_todos.len());
    
    let todo_data = TodoData {
        pending_todos,
        completed_todos,
    };
    
    let json_data = serde_json::to_string_pretty(&todo_data)
        .map_err(|e| {
            log::error!("序列化数据失败: {}", e);
            format!("序列化数据失败: {}", e)
        })?;
    
    fs::write(&file_path, json_data)
        .map_err(|e| {
            log::error!("写入待办事项文件失败: {}", e);
            format!("写入文件失败: {}", e)
        })?;
    
    log::info!("待办事项数据保存成功");
    Ok(())
}

// Tauri 命令：加载todo数据
#[tauri::command]
pub async fn load_todo_data(app: tauri::AppHandle) -> Result<TodoData, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("todo_list.json");
    
    if !file_path.exists() {
        // 如果文件不存在，返回空数据
        log::info!("待办事项文件不存在，返回空数据");
        return Ok(TodoData {
            pending_todos: vec![],
            completed_todos: vec![],
        });
    }
    
    log::info!("加载待办事项数据");
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| {
            log::error!("读取待办事项文件失败: {}", e);
            format!("读取文件失败: {}", e)
        })?;
    
    let todo_data: TodoData = serde_json::from_str(&json_data)
        .map_err(|e| {
            log::error!("解析待办事项JSON失败: {}", e);
            format!("解析JSON失败: {}", e)
        })?;
    
    log::info!("待办事项数据加载成功: 待完成={}, 已完成={}", 
        todo_data.pending_todos.len(), todo_data.completed_todos.len());
    
    Ok(todo_data)
}

// Tauri 命令：更新todo文本内容
#[tauri::command]
pub async fn update_todo_text(
    app: tauri::AppHandle,
    todo_id: String,
    is_completed: bool,
    new_text: String
) -> Result<(), String> {
    log::info!("更新任务文本: id='{}', completed={}, new_text='{}'", 
        todo_id, is_completed, new_text);
    
    // 先加载当前数据
    let mut todo_data = load_todo_data(app.clone()).await?;
    
    log::debug!("加载的数据: pending_count={}, completed_count={}", 
        todo_data.pending_todos.len(), todo_data.completed_todos.len());
    
    // 查找并更新对应的todo项
    let found = if is_completed {
        // 在已完成列表中查找
        log::debug!("在已完成列表中查找任务");
        todo_data.completed_todos.iter_mut()
            .find(|todo| todo.id == todo_id)
    } else {
        // 在待完成列表中查找
        log::debug!("在待完成列表中查找任务");
        todo_data.pending_todos.iter_mut()
            .find(|todo| todo.id == todo_id)
    };
    
    if let Some(todo) = found {
        log::info!("找到任务，更新文本");
        let old_text = todo.text.clone();
        todo.text = new_text.clone();
        
        // 先克隆数据，避免借用冲突
        let pending_clone = todo_data.pending_todos.clone();
        let completed_clone = todo_data.completed_todos.clone();
        
        // 保存更新后的数据
        let save_result = save_todo_data(app, pending_clone, completed_clone).await;
        
        if save_result.is_err() {
            log::error!("保存任务文本失败");
            return save_result;
        }
        
        log::info!("任务文本更新成功: '{}' -> '{}'", old_text, new_text);
        Ok(())
    } else {
        let error_msg = format!("未找到指定的todo项: id='{}', completed={}", todo_id, is_completed);
        log::error!("{}", error_msg);
        Err(error_msg)
    }
}

// Tauri 命令：设置todo截止时间
#[tauri::command]
pub async fn set_todo_deadline(
    app: tauri::AppHandle, 
    todo_id: String,  // 使用ID而不是文本
    is_completed: bool,
    deadline: Option<i64>
) -> Result<(), String> {
    log::info!("设置截止时间: id='{}', completed={}, deadline={:?}", 
        todo_id, is_completed, deadline);
    
    // 先加载当前数据
    let mut todo_data = load_todo_data(app.clone()).await?;
    
    log::debug!("加载的数据: pending_count={}, completed_count={}", 
        todo_data.pending_todos.len(), todo_data.completed_todos.len());
    
    // 查找并更新对应的todo项
    let found = if is_completed {
        // 在已完成列表中查找
        log::debug!("在已完成列表中查找任务");
        todo_data.completed_todos.iter_mut()
            .find(|todo| todo.id == todo_id)
    } else {
        // 在待完成列表中查找
        log::debug!("在待完成列表中查找任务");
        todo_data.pending_todos.iter_mut()
            .find(|todo| todo.id == todo_id)
    };
    
    if let Some(todo) = found {
        log::info!("找到任务，更新截止时间");
        let old_deadline = todo.deadline;
        todo.deadline = deadline;
        
        // 保存更新后的数据
        let save_result = save_todo_data(app, todo_data.pending_todos.clone(), todo_data.completed_todos.clone()).await;
        
        if save_result.is_err() {
            log::error!("保存截止时间失败");
            return save_result;
        }
        
        // 根据deadline值提供不同的成功消息
        if deadline.is_some() {
            log::info!("截止时间设置成功: {:?} -> {:?}", old_deadline, deadline);
        } else {
            log::info!("截止时间移除成功");
        }
        Ok(())
    } else {
        let error_msg = format!("未找到指定的todo项: id='{}', completed={}", todo_id, is_completed);
        log::error!("{}", error_msg);
        Err(error_msg)
    }
}

// Tauri 命令：保存带分组的todo数据
#[tauri::command]
pub async fn save_todo_data_with_groups(app: tauri::AppHandle, todos: Vec<Todo>) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("todos_with_groups.json");
    
    log::info!("保存带分组的待办事项数据: 总数={}", todos.len());
    
    let todo_data = crate::models::TodoDataWithGroups {
        todos,
    };
    
    let json_data = serde_json::to_string_pretty(&todo_data)
        .map_err(|e| {
            log::error!("序列化分组数据失败: {}", e);
            format!("序列化数据失败: {}", e)
        })?;
    
    fs::write(&file_path, json_data)
        .map_err(|e| {
            log::error!("写入分组数据文件失败: {}", e);
            format!("写入文件失败: {}", e)
        })?;
    
    log::info!("分组数据保存成功");
    Ok(())
}

// Tauri 命令：加载带分组的todo数据
#[tauri::command]
pub async fn load_todo_data_with_groups(app: tauri::AppHandle) -> Result<crate::models::TodoDataWithGroups, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("todos_with_groups.json");
    
    if !file_path.exists() {
        // 如果文件不存在，返回空数据
        log::info!("分组数据文件不存在，返回空数据");
        return Ok(crate::models::TodoDataWithGroups {
            todos: Vec::new(),
        });
    }
    
    log::info!("加载带分组的待办事项数据");
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| {
            log::error!("读取分组数据文件失败: {}", e);
            format!("读取文件失败: {}", e)
        })?;
    
    let todo_data: crate::models::TodoDataWithGroups = serde_json::from_str(&json_data)
        .map_err(|e| {
            log::error!("解析分组数据JSON失败: {}", e);
            format!("解析JSON失败: {}", e)
        })?;
    
    log::info!("分组数据加载成功: 总数={}", todo_data.todos.len());
    Ok(todo_data)
}

// Tauri 命令：更新单个todo（修改指定字段，不重写整个文件）
#[tauri::command]
pub async fn update_single_todo(app: tauri::AppHandle, todo: crate::models::Todo) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("todos_with_groups.json");

    // 读取现有数据
    let mut todo_data: crate::models::TodoDataWithGroups = if file_path.exists() {
        let json_data = fs::read_to_string(&file_path)
            .map_err(|e| format!("读取文件失败: {}", e))?;
        serde_json::from_str(&json_data)
            .map_err(|e| format!("解析JSON失败: {}", e))?
    } else {
        crate::models::TodoDataWithGroups { todos: Vec::new() }
    };

    // 查找并替换对应todo
    if let Some(existing) = todo_data.todos.iter_mut().find(|t| t.id == todo.id) {
        *existing = todo;
    } else {
        todo_data.todos.push(todo);
    }

    // 写回文件
    let json_data = serde_json::to_string_pretty(&todo_data)
        .map_err(|e| format!("序列化数据失败: {}", e))?;
    fs::write(&file_path, json_data)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

// Tauri 命令：更新单个分组
#[tauri::command]
pub async fn update_single_group(app: tauri::AppHandle, group: crate::models::TodoGroup) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("groups.json");

    // 读取现有数据
    let mut group_data: crate::models::GroupData = if file_path.exists() {
        let json_data = fs::read_to_string(&file_path)
            .map_err(|e| format!("读取文件失败: {}", e))?;
        serde_json::from_str(&json_data)
            .map_err(|e| format!("解析JSON失败: {}", e))?
    } else {
        crate::models::GroupData { groups: Vec::new() }
    };

    // 查找并替换对应分组
    if let Some(existing) = group_data.groups.iter_mut().find(|g| g.id == group.id) {
        *existing = group;
    } else {
        group_data.groups.push(group);
    }

    // 写回文件
    let json_data = serde_json::to_string_pretty(&group_data)
        .map_err(|e| format!("序列化数据失败: {}", e))?;
    fs::write(&file_path, json_data)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

// Tauri 命令：保存分组数据
#[tauri::command]
pub async fn save_group_data(app: tauri::AppHandle, groups: Vec<crate::models::TodoGroup>) -> Result<(), String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("groups.json");
    
    log::info!("保存分组数据: 分组数={}", groups.len());
    
    let group_data = crate::models::GroupData {
        groups,
    };
    
    let json_data = serde_json::to_string_pretty(&group_data)
        .map_err(|e| {
            log::error!("序列化分组数据失败: {}", e);
            format!("序列化数据失败: {}", e)
        })?;
    
    fs::write(&file_path, json_data)
        .map_err(|e| {
            log::error!("写入分组数据文件失败: {}", e);
            format!("写入文件失败: {}", e)
        })?;
    
    log::info!("分组数据保存成功");
    Ok(())
}

// Tauri 命令：加载分组数据
#[tauri::command]
pub async fn load_group_data(app: tauri::AppHandle) -> Result<crate::models::GroupData, String> {
    let data_dir = get_data_dir(&app)?;
    let file_path = data_dir.join("groups.json");
    
    if !file_path.exists() {
        // 如果文件不存在，返回默认分组
        log::info!("分组数据文件不存在，使用默认分组");
        return Ok(crate::models::GroupData {
            groups: vec![crate::models::TodoGroup {
                id: "default".to_string(),
                name: "未分组".to_string(),
                order: 0,
                collapsed: false,
                updated_at: 0,
            }],
        });
    }
    
    log::info!("加载分组数据");
    
    let json_data = fs::read_to_string(&file_path)
        .map_err(|e| {
            log::error!("读取分组数据文件失败: {}", e);
            format!("读取文件失败: {}", e)
        })?;
    
    let group_data: crate::models::GroupData = serde_json::from_str(&json_data)
        .map_err(|e| {
            log::error!("解析分组数据JSON失败: {}", e);
            format!("解析JSON失败: {}", e)
        })?;
    
    log::info!("分组数据加载成功: 分组数={}", group_data.groups.len());
    Ok(group_data)
}
