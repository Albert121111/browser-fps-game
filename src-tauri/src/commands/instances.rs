use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Instance {
  pub id: String,
  pub name: String,
  pub minecraft_version: String,
  pub loader: String,
  pub loader_version: String,
  pub java: String,
  pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
  pub name: String,
  pub minecraft_version: String,
  pub loader: String,
  pub loader_version: String,
  pub java: String,
  pub path: String,
}

#[tauri::command]
pub fn list_instances() -> Vec<Instance> {
  vec![
    Instance {
      id: "vanilla-plus".into(),
      name: "Vanilla+".into(),
      minecraft_version: "1.20.4".into(),
      loader: "fabric".into(),
      loader_version: "0.15.6".into(),
      java: "auto".into(),
      path: "~/Games/Mintcraft/Instances/Vanilla".into(),
    },
    Instance {
      id: "emerald".into(),
      name: "Emerald Horizons".into(),
      minecraft_version: "1.20.4".into(),
      loader: "fabric".into(),
      loader_version: "0.15.6".into(),
      java: "temurin-17".into(),
      path: "~/Games/Mintcraft/Instances/Emerald".into(),
    },
  ]
}

#[tauri::command]
pub fn create_instance(request: CreateInstanceRequest) -> Instance {
  Instance {
    id: request.name.to_lowercase().replace(' ', "-"),
    name: request.name,
    minecraft_version: request.minecraft_version,
    loader: request.loader,
    loader_version: request.loader_version,
    java: request.java,
    path: request.path,
  }
}

#[tauri::command]
pub fn duplicate_instance(instance_id: String, new_name: String) -> Instance {
  Instance {
    id: format!("{}-copy", instance_id),
    name: new_name,
    minecraft_version: "1.20.4".into(),
    loader: "fabric".into(),
    loader_version: "0.15.6".into(),
    java: "auto".into(),
    path: "~/Games/Mintcraft/Instances/Copy".into(),
  }
}

#[tauri::command]
pub fn delete_instance(_instance_id: String) -> bool {
  true
}

#[tauri::command]
pub fn export_instance(_instance_id: String) -> String {
  "~/Downloads/instance.zip".into()
}
