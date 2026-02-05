use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
  pub user_code: String,
  pub device_code: String,
  pub verification_uri: String,
  pub expires_in: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokens {
  pub access_token: String,
  pub refresh_token: String,
  pub expires_in: u32,
}

#[tauri::command]
pub async fn begin_device_flow() -> Result<DeviceCodeResponse, String> {
  // TODO: integrate Microsoft device code flow.
  Ok(DeviceCodeResponse {
    user_code: "ABCD-EFGH".into(),
    device_code: "device-code".into(),
    verification_uri: "https://microsoft.com/devicelogin".into(),
    expires_in: 900,
  })
}

#[tauri::command]
pub async fn refresh_token(_refresh_token: String) -> Result<AuthTokens, String> {
  Ok(AuthTokens {
    access_token: "access-token".into(),
    refresh_token: "refresh-token".into(),
    expires_in: 3600,
  })
}
