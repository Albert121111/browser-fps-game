use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MrPackManifest {
  pub format_version: u32,
  pub name: String,
}

pub fn validate_zip_paths(paths: &[String]) -> bool {
  paths.iter().all(|path| {
    !path.contains("..") && !path.starts_with('/') && !path.starts_with('\\')
  })
}

pub fn parse_mrpack_manifest(payload: &str) -> Result<MrPackManifest, serde_json::Error> {
  serde_json::from_str(payload)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rejects_path_traversal() {
    let ok = vec!["mods/test.jar".to_string()];
    let bad = vec!["../evil".to_string()];
    assert!(validate_zip_paths(&ok));
    assert!(!validate_zip_paths(&bad));
  }

  #[test]
  fn parses_mrpack_manifest() {
    let payload = r#"{\"format_version\":1,\"name\":\"Test\"}"#;
    let manifest = parse_mrpack_manifest(payload).unwrap();
    assert_eq!(manifest.name, "Test");
  }
}
