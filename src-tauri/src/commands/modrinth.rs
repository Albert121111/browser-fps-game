use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthProject {
  pub project_id: String,
  pub title: String,
  pub project_type: String,
}

#[tauri::command]
pub async fn list_modrinth_projects(query: String) -> Result<Vec<ModrinthProject>, String> {
  let _endpoint = format!(
    "https://api.modrinth.com/v2/search?query={}",
    urlencoding::encode(&query)
  );
  Ok(vec![ModrinthProject {
    project_id: "sodium".into(),
    title: "Sodium".into(),
    project_type: "mod".into(),
  }])
}

#[tauri::command]
pub async fn install_modrinth_project(_project_id: String) -> Result<bool, String> {
  Ok(true)
}
