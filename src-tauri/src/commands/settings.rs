use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
  pub min_ram: u32,
  pub max_ram: u32,
  pub java_path: Option<String>,
  pub jvm_args: String,
  pub fullscreen: bool,
  pub resolution: String,
  pub instances_dir: String,
  pub data_dir: String,
  pub language: String,
  pub theme: String,
}

#[tauri::command]
pub fn update_settings(settings: Settings) -> Settings {
  settings
}
