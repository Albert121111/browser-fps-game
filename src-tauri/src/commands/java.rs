use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JavaInfo {
  pub version: String,
  pub path: String,
  pub source: String,
}

#[tauri::command]
pub fn resolve_java(_minecraft_version: String, _loader: String) -> JavaInfo {
  JavaInfo {
    version: "17".into(),
    path: "auto".into(),
    source: "system".into(),
  }
}
