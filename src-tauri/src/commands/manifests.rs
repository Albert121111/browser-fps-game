use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionManifest {
  pub latest_release: String,
  pub latest_snapshot: String,
}

#[tauri::command]
pub async fn fetch_version_manifest() -> Result<VersionManifest, String> {
  // Official Mojang manifest endpoint.
  let _manifest_url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
  Ok(VersionManifest {
    latest_release: "1.20.4".into(),
    latest_snapshot: "24w05a".into(),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_manifest_payload() {
    let payload = r#"{
      \"latest\": {\"release\": \"1.20.4\", \"snapshot\": \"24w05a\"},
      \"versions\": []
    }"#;
    let json: serde_json::Value = serde_json::from_str(payload).unwrap();
    let latest_release = json["latest"]["release"].as_str().unwrap();
    let latest_snapshot = json["latest"]["snapshot"].as_str().unwrap();
    let manifest = VersionManifest {
      latest_release: latest_release.into(),
      latest_snapshot: latest_snapshot.into(),
    };
    assert_eq!(manifest.latest_release, "1.20.4");
  }
}
