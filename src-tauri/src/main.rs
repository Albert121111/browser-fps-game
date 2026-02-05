mod commands;
mod state;

use commands::*;
use state::AppState;

fn main() {
  tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
      list_instances,
      create_instance,
      duplicate_instance,
      delete_instance,
      export_instance,
      fetch_version_manifest,
      install_minecraft,
      install_loader,
      begin_device_flow,
      refresh_token,
      resolve_java,
      build_launch_args,
      list_modrinth_projects,
      install_modrinth_project,
      list_downloads,
      enqueue_download,
      cancel_download,
      update_settings
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
