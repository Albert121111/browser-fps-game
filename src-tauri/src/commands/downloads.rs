use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadTask {
  pub id: String,
  pub label: String,
  pub progress: f32,
  pub status: String,
}

#[tauri::command]
pub fn list_downloads(state: tauri::State<AppState>) -> Vec<DownloadTask> {
  state.downloads.lock().unwrap().clone()
}

#[tauri::command]
pub fn enqueue_download(state: tauri::State<AppState>, label: String) -> DownloadTask {
  let task = DownloadTask {
    id: format!("task-{}", label.to_lowercase().replace(' ', "-")),
    label,
    progress: 0.0,
    status: "queued".into(),
  };
  state.downloads.lock().unwrap().push(task.clone());
  task
}

#[tauri::command]
pub fn cancel_download(state: tauri::State<AppState>, task_id: String) -> bool {
  let mut tasks = state.downloads.lock().unwrap();
  let before = tasks.len();
  tasks.retain(|task| task.id != task_id);
  before != tasks.len()
}
