use crate::{
    error::{ParadexError, Result},
    types::{AuthResponse, SystemConfig},
};
use reqwest::Client;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

/// Perform onboarding for a new account
pub async fn onboard(
    client: &Client,
    api_url: &str,
    headers: Vec<(String, String)>,
    public_key: &str,
) -> Result<()> {
    let url = format!("{}/onboarding", api_url);
    let payload = json!({
        "public_key": public_key
    });

    let mut request = client.post(&url).json(&payload);

    // Add custom headers
    for (key, value) in headers {
        request = request.header(key, value);
    }

    let response = request.send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

        // Onboarding might fail if already onboarded - that's ok
        if status.as_u16() == 400 && error_text.contains("already") {
            log::debug!("Account already onboarded");
            Ok(())
        } else {
            Err(ParadexError::ApiError {
                status: status.as_u16(),
                message: error_text,
            })
        }
    }
}

/// Authenticate and get JWT token
pub async fn authenticate(
    client: &Client,
    api_url: &str,
    headers: Vec<(String, String)>,
    public_key: &str,
) -> Result<String> {
    let url = format!("{}/auth/{}", api_url, public_key);

    let mut request = client.post(&url);

    // Add custom headers
    for (key, value) in headers {
        request = request.header(key, value);
    }

    let response = request.send().await?;

    if response.status().is_success() {
        let auth_response: AuthResponse = response.json().await?;
        Ok(auth_response.jwt_token)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(ParadexError::ApiError {
            status: status.as_u16(),
            message: error_text,
        })
    }
}

/// Check if JWT token needs refresh (older than 4 minutes)
pub fn needs_refresh(auth_timestamp: SystemTime) -> bool {
    let now = SystemTime::now();
    let elapsed = now.duration_since(auth_timestamp).unwrap_or_default();
    elapsed.as_secs() > 4 * 60 // 4 minutes
}

/// Get current timestamp
pub fn current_timestamp() -> SystemTime {
    SystemTime::now()
}

/// Get timestamp as seconds since epoch
pub fn timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needs_refresh() {
        let old_time = SystemTime::now() - std::time::Duration::from_secs(5 * 60);
        assert!(needs_refresh(old_time));

        let recent_time = SystemTime::now() - std::time::Duration::from_secs(60);
        assert!(!needs_refresh(recent_time));
    }

    #[test]
    fn test_timestamp_secs() {
        let ts = timestamp_secs();
        assert!(ts > 0);
    }
}
