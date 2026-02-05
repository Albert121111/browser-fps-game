use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallRequest {
  pub minecraft_version: String,
  pub loader: String,
  pub loader_version: String,
  pub instance_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallEvent {
  pub stage: String,
  pub progress: f32,
  pub message: String,
}

#[tauri::command]
pub async fn install_minecraft(
  window: tauri::Window,
  request: InstallRequest,
) -> Result<(), String> {
  let _ = window.emit(
    "download-progress",
    InstallEvent {
      stage: "assets".into(),
      progress: 0.35,
      message: format!("Installing {}", request.minecraft_version),
    },
  );
  Ok(())
}

#[tauri::command]
pub async fn install_loader(
  window: tauri::Window,
  request: InstallRequest,
) -> Result<(), String> {
  let _ = window.emit(
    "download-progress",
    InstallEvent {
      stage: "loader".into(),
      progress: 0.5,
      message: format!("Installing {} {}", request.loader, request.loader_version),
    },
  );
  Ok(())
}
