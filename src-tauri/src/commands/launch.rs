use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchRequest {
  pub instance_id: String,
  pub java_path: String,
  pub min_memory: u32,
  pub max_memory: u32,
  pub loader: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchArgs {
  pub jvm_args: Vec<String>,
  pub game_args: Vec<String>,
}

#[tauri::command]
pub fn build_launch_args(request: LaunchRequest) -> LaunchArgs {
  let jvm_args = build_jvm_args(request.min_memory, request.max_memory, &request.java_path);
  let game_args = vec!["--username".into(), "Player".into()];
  LaunchArgs { jvm_args, game_args }
}

pub fn build_jvm_args(min_memory: u32, max_memory: u32, java_path: &str) -> Vec<String> {
  vec![
    format!("-Xms{}M", min_memory),
    format!("-Xmx{}M", max_memory),
    format!("-Djava.home={}", java_path),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn builds_jvm_args() {
    let args = build_jvm_args(1024, 4096, "/java");
    assert!(args.iter().any(|arg| arg == "-Xms1024M"));
    assert!(args.iter().any(|arg| arg == "-Xmx4096M"));
  }
}
