use chrono::Local;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::task::Task;

pub struct Scheduler {
    handles: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            handles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Schedule a notification for a task at the specified time
    pub async fn schedule_task(&self, app: AppHandle, task: &Task) {
        if task.done || task.notified {
            return;
        }

        let notify_at = match &task.notify_at {
            Some(t) => t.clone(),
            None => return,
        };

        let task_id = task.id.clone();
        let task_text = task.text.clone();
        let notify_time = notify_at.clone();

        // Calculate delay
        let now = Local::now();
        let parts: Vec<&str> = notify_time.split(':').collect();
        if parts.len() != 2 {
            return;
        }

        let target_hour: u32 = match parts[0].parse() {
            Ok(h) => h,
            Err(_) => return,
        };
        let target_min: u32 = match parts[1].parse() {
            Ok(m) => m,
            Err(_) => return,
        };

        let target = now
            .date_naive()
            .and_hms_opt(target_hour, target_min, 0);

        let target = match target {
            Some(t) => t,
            None => return,
        };

        let target_dt = target.and_local_timezone(Local).single();
        let target_dt = match target_dt {
            Some(t) => t,
            None => return,
        };

        // If the time has already passed today, skip
        if target_dt <= now {
            return;
        }

        let delay = (target_dt - now).to_std().unwrap_or_default();

        let handles = self.handles.clone();
        let task_id_clone = task_id.clone();

        let handle = tokio::spawn(async move {
            tokio::time::sleep(delay).await;

            // Send notification
            let _ = app
                .notification()
                .builder()
                .title("QuickTask")
                .body(&format!("⏰ {} ({})", task_text, notify_time))
                .show();

            // Clean up handle
            let mut handles = handles.lock().await;
            handles.remove(&task_id_clone);
        });

        let mut handles = self.handles.lock().await;
        // Cancel existing timer for the same task
        if let Some(old_handle) = handles.remove(&task_id) {
            old_handle.abort();
        }
        handles.insert(task_id, handle);
    }

    /// Cancel a scheduled notification
    pub async fn cancel_task(&self, task_id: &str) {
        let mut handles = self.handles.lock().await;
        if let Some(handle) = handles.remove(task_id) {
            handle.abort();
        }
    }

    /// Cancel all scheduled notifications
    pub async fn cancel_all(&self) {
        let mut handles = self.handles.lock().await;
        for (_, handle) in handles.drain() {
            handle.abort();
        }
    }
}
