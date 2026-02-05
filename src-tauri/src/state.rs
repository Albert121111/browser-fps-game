use std::sync::Mutex;

use crate::commands::downloads::DownloadTask;

#[derive(Default)]
pub struct AppState {
  pub downloads: Mutex<Vec<DownloadTask>>,
}
