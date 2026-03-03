mod scheduler;
mod task;

use scheduler::Scheduler;
use serde_json;
use std::fs;
use std::sync::Arc;
use task::{parse_task_input, Task, TaskStore};
use tauri::{
    AppHandle, Manager, State,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tokio::sync::Mutex;

pub struct AppState {
    pub store: Arc<Mutex<TaskStore>>,
    pub scheduler: Arc<Scheduler>,
}

fn get_tasks_path(app: &AppHandle) -> std::path::PathBuf {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    fs::create_dir_all(&app_dir).ok();
    app_dir.join("tasks.json")
}

fn load_tasks(app: &AppHandle) -> TaskStore {
    let path = get_tasks_path(app);
    if path.exists() {
        let data = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_else(|_| TaskStore::new())
    } else {
        TaskStore::new()
    }
}

fn save_tasks(app: &AppHandle, store: &TaskStore) {
    let path = get_tasks_path(app);
    let data = serde_json::to_string_pretty(store).unwrap_or_default();
    fs::write(path, data).ok();
}

#[tauri::command]
async fn add_task(
    app: AppHandle,
    state: State<'_, AppState>,
    input: String,
) -> Result<Task, String> {
    let (text, notify_at) = parse_task_input(&input);

    if text.is_empty() {
        return Err("Task text is empty".to_string());
    }

    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        text,
        done: false,
        notify_at,
        notified: false,
        created_at: chrono::Local::now().to_rfc3339(),
    };

    // Schedule notification if time is set
    state
        .scheduler
        .schedule_task(app.clone(), &task)
        .await;

    let mut store = state.store.lock().await;
    store.tasks.push(task.clone());
    save_tasks(&app, &store);

    Ok(task)
}

#[tauri::command]
async fn get_tasks(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<Task>, String> {
    let store = state.store.lock().await;
    save_tasks(&app, &store); // Ensure file exists
    Ok(store.tasks.clone())
}

#[tauri::command]
async fn toggle_task(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<Task, String> {
    let mut store = state.store.lock().await;
    let task = store
        .tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or("Task not found")?;

    task.done = !task.done;
    let updated = task.clone();

    if updated.done {
        state.scheduler.cancel_task(&id).await;
    } else {
        // Re-schedule if unchecked
        state
            .scheduler
            .schedule_task(app.clone(), &updated)
            .await;
    }

    save_tasks(&app, &store);
    Ok(updated)
}

#[tauri::command]
async fn delete_task(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let mut store = state.store.lock().await;
    store.tasks.retain(|t| t.id != id);
    state.scheduler.cancel_task(&id).await;
    save_tasks(&app, &store);
    Ok(())
}

#[tauri::command]
async fn clear_all_tasks(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut store = state.store.lock().await;
    store.tasks.clear();
    state.scheduler.cancel_all().await;
    save_tasks(&app, &store);
    Ok(())
}

fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit])?;

    let _tray = TrayIconBuilder::with_id("quicktask-tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("QuickTask")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                button_state: tauri::tray::MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let handle = app.handle().clone();

            // Load tasks
            let store = load_tasks(&handle);

            // Create scheduler
            let scheduler = Arc::new(Scheduler::new());

            // Schedule existing tasks
            let sched_clone = scheduler.clone();
            let handle_clone = handle.clone();
            let tasks_clone = store.tasks.clone();
            tauri::async_runtime::spawn(async move {
                for task in &tasks_clone {
                    sched_clone
                        .schedule_task(handle_clone.clone(), task)
                        .await;
                }
            });

            // Store state
            app.manage(AppState {
                store: Arc::new(Mutex::new(store)),
                scheduler,
            });

            // Setup system tray
            setup_tray(&handle)?;

            // Hide to tray on close instead of quitting
            let window = app.get_webview_window("main").unwrap();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    if let Some(w) = handle.get_webview_window("main") {
                        let _ = w.hide();
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_task,
            get_tasks,
            toggle_task,
            delete_task,
            clear_all_tasks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
