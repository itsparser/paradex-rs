use crate::{environment::Environment, error::Result};
use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;

/// HTTP client for making requests to Paradex API
pub struct HttpClient {
    client: Client,
    api_url: String,
    jwt_token: Option<String>,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new(env: Environment) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            api_url: env.api_url(),
            jwt_token: None,
        })
    }

    /// Set JWT token for authenticated requests
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.jwt_token = Some(token.into());
    }

    /// Get the underlying reqwest client
    pub(crate) fn get_client(&self) -> Client {
        self.client.clone()
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", self.api_url, path);
        let mut request = self.client.get(&url);
        request = self.add_auth_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a GET request with query parameters
    pub async fn get_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}/{}", self.api_url, path);
        let mut request = self.client.get(&url).query(params);
        request = self.add_auth_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a POST request
    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}/{}", self.api_url, path);
        let mut request = self.client.post(&url).json(body);
        request = self.add_auth_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a PUT request
    pub async fn put<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}/{}", self.api_url, path);
        let mut request = self.client.put(&url).json(body);
        request = self.add_auth_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", self.api_url, path);
        let mut request = self.client.delete(&url);
        request = self.add_auth_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a DELETE request with body
    pub async fn delete_with_body<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}/{}", self.api_url, path);
        let mut request = self.client.delete(&url).json(body);
        request = self.add_auth_header(request);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    fn add_auth_header(&self, request: RequestBuilder) -> RequestBuilder {
        if let Some(token) = &self.jwt_token {
            request.bearer_auth(token)
        } else {
            request
        }
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::error::ParadexError::ApiError {
                status: status.as_u16(),
                message: error_text,
            })
        }
    }
}
