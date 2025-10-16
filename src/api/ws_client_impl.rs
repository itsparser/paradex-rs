use crate::{
    environment::Environment,
    error::{ParadexError, Result},
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type MessageCallback =
    Arc<dyn Fn(serde_json::Value) -> futures::future::BoxFuture<'static, ()> + Send + Sync>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WsRequest {
    id: u64,
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WsResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<serde_json::Value>,
}

/// WebSocket client implementation with full channel support
pub struct WebSocketClientImpl {
    ws_url: String,
    jwt_token: Option<String>,
    ws_stream: Arc<Mutex<Option<WsStream>>>,
    callbacks: Arc<RwLock<HashMap<String, MessageCallback>>>,
    subscribed_channels: Arc<RwLock<HashMap<String, bool>>>,
    next_id: Arc<Mutex<u64>>,
    is_connected: Arc<Mutex<bool>>,
    auto_reconnect: bool,
    ping_interval: Option<Duration>,
}

impl WebSocketClientImpl {
    /// Create a new WebSocket client
    pub fn new(env: Environment) -> Self {
        Self {
            ws_url: env.ws_url(),
            jwt_token: None,
            ws_stream: Arc::new(Mutex::new(None)),
            callbacks: Arc::new(RwLock::new(HashMap::new())),
            subscribed_channels: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            is_connected: Arc::new(Mutex::new(false)),
            auto_reconnect: true,
            ping_interval: Some(Duration::from_secs(20)),
        }
    }

    /// Set JWT token for authenticated channels
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.jwt_token = Some(token.into());
    }

    /// Connect to WebSocket
    pub async fn connect(&self) -> Result<()> {
        let url = &self.ws_url;
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| ParadexError::WebSocketError(format!("Connection failed: {}", e)))?;

        *self.ws_stream.lock().await = Some(ws_stream);
        *self.is_connected.lock().await = true;

        // Authenticate if we have a token
        if let Some(token) = &self.jwt_token {
            self.send_auth(token).await?;
        }

        // Start message reader in background
        let stream_clone = Arc::clone(&self.ws_stream);
        let callbacks_clone = Arc::clone(&self.callbacks);
        let is_connected_clone = Arc::clone(&self.is_connected);
        let auto_reconnect = self.auto_reconnect;

        tokio::spawn(async move {
            Self::read_messages(
                stream_clone,
                callbacks_clone,
                is_connected_clone,
                auto_reconnect,
            )
            .await;
        });

        // Start ping task if configured
        if let Some(interval) = self.ping_interval {
            let stream_clone = Arc::clone(&self.ws_stream);
            let is_connected_clone = Arc::clone(&self.is_connected);

            tokio::spawn(async move {
                Self::ping_loop(stream_clone, is_connected_clone, interval).await;
            });
        }

        log::info!("WebSocket connected to {}", url);
        Ok(())
    }

    /// Send authentication message
    async fn send_auth(&self, token: &str) -> Result<()> {
        let auth_msg = json!({
            "id": self.get_next_id().await,
            "jsonrpc": "2.0",
            "method": "auth",
            "params": {
                "bearer": token
            }
        });

        self.send_message(&auth_msg).await?;
        log::info!("Sent authentication message");
        Ok(())
    }

    /// Subscribe to a channel
    pub async fn subscribe(&self, channel: &str, callback: MessageCallback) -> Result<()> {
        // Store callback
        self.callbacks
            .write()
            .await
            .insert(channel.to_string(), callback);

        // Send subscription message
        let sub_msg = json!({
            "id": self.get_next_id().await,
            "jsonrpc": "2.0",
            "method": "subscribe",
            "params": {
                "channel": channel
            }
        });

        self.send_message(&sub_msg).await?;
        self.subscribed_channels
            .write()
            .await
            .insert(channel.to_string(), true);

        log::info!("Subscribed to channel: {}", channel);
        Ok(())
    }

    /// Unsubscribe from a channel
    pub async fn unsubscribe(&self, channel: &str) -> Result<()> {
        let unsub_msg = json!({
            "id": self.get_next_id().await,
            "jsonrpc": "2.0",
            "method": "unsubscribe",
            "params": {
                "channel": channel
            }
        });

        self.send_message(&unsub_msg).await?;
        self.subscribed_channels.write().await.remove(channel);
        self.callbacks.write().await.remove(channel);

        log::info!("Unsubscribed from channel: {}", channel);
        Ok(())
    }

    /// Send a message to the WebSocket
    async fn send_message(&self, message: &serde_json::Value) -> Result<()> {
        let msg_str = serde_json::to_string(message)?;
        let mut stream = self.ws_stream.lock().await;

        if let Some(ws) = stream.as_mut() {
            ws.send(Message::Text(msg_str))
                .await
                .map_err(|e| ParadexError::WebSocketError(format!("Send failed: {}", e)))?;
            Ok(())
        } else {
            Err(ParadexError::WebSocketError("Not connected".to_string()))
        }
    }

    /// Read messages from WebSocket in a loop
    async fn read_messages(
        stream: Arc<Mutex<Option<WsStream>>>,
        callbacks: Arc<RwLock<HashMap<String, MessageCallback>>>,
        is_connected: Arc<Mutex<bool>>,
        _auto_reconnect: bool,
    ) {
        loop {
            let mut stream_guard = stream.lock().await;

            if let Some(ws) = stream_guard.as_mut() {
                match ws.next().await {
                    Some(Ok(Message::Text(text))) => {
                        drop(stream_guard);

                        if let Ok(response) = serde_json::from_str::<WsResponse>(&text) {
                            if let Some(params) = response.params {
                                if let Some(channel) =
                                    params.get("channel").and_then(|v| v.as_str())
                                {
                                    let callbacks_read = callbacks.read().await;
                                    if let Some(callback) = callbacks_read.get(channel) {
                                        let callback_clone = Arc::clone(callback);
                                        let params_clone = params.clone();
                                        tokio::spawn(async move {
                                            callback_clone(params_clone).await;
                                        });
                                    }
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        log::info!("WebSocket closed");
                        *is_connected.lock().await = false;
                        break;
                    }
                    Some(Err(e)) => {
                        log::error!("WebSocket error: {}", e);
                        *is_connected.lock().await = false;
                        break;
                    }
                    None => {
                        log::info!("WebSocket stream ended");
                        *is_connected.lock().await = false;
                        break;
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }
    }

    /// Ping loop to keep connection alive
    async fn ping_loop(
        stream: Arc<Mutex<Option<WsStream>>>,
        is_connected: Arc<Mutex<bool>>,
        interval: Duration,
    ) {
        loop {
            sleep(interval).await;

            if !*is_connected.lock().await {
                break;
            }

            let mut stream_guard = stream.lock().await;
            if let Some(ws) = stream_guard.as_mut() {
                if ws.send(Message::Ping(vec![])).await.is_err() {
                    log::error!("Failed to send ping");
                    break;
                }
            }
        }
    }

    /// Get next request ID
    async fn get_next_id(&self) -> u64 {
        let mut id = self.next_id.lock().await;
        let current = *id;
        *id += 1;
        current
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        *self.is_connected.lock().await
    }

    /// Close the WebSocket connection
    pub async fn close(&self) -> Result<()> {
        let mut stream = self.ws_stream.lock().await;
        if let Some(ws) = stream.as_mut() {
            ws.close(None)
                .await
                .map_err(|e| ParadexError::WebSocketError(format!("Close failed: {}", e)))?;
        }
        *self.is_connected.lock().await = false;
        Ok(())
    }

    /// Get current subscriptions
    pub async fn get_subscriptions(&self) -> HashMap<String, bool> {
        self.subscribed_channels.read().await.clone()
    }

    /// Manually pump one message from the WebSocket (for deterministic consumption)
    pub async fn pump_once(&self) -> Result<bool> {
        let mut stream_guard = self.ws_stream.lock().await;

        if let Some(ws) = stream_guard.as_mut() {
            match tokio::time::timeout(Duration::from_millis(1), ws.next()).await {
                Ok(Some(Ok(Message::Text(text)))) => {
                    drop(stream_guard);

                    if let Ok(response) = serde_json::from_str::<WsResponse>(&text) {
                        if let Some(params) = response.params {
                            if let Some(channel) = params.get("channel").and_then(|v| v.as_str()) {
                                let callbacks_read = self.callbacks.read().await;
                                if let Some(callback) = callbacks_read.get(channel) {
                                    let callback_clone = Arc::clone(callback);
                                    let params_clone = params.clone();
                                    tokio::spawn(async move {
                                        callback_clone(params_clone).await;
                                    });
                                }
                            }
                        }
                    }
                    Ok(true)
                }
                Ok(None) | Err(_) => Ok(false),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    /// Pump messages until predicate is satisfied or timeout
    pub async fn pump_until<F>(&self, predicate: F, timeout_secs: f64) -> Result<u32>
    where
        F: Fn(&serde_json::Value) -> bool,
    {
        let start = std::time::Instant::now();
        let timeout = Duration::from_secs_f64(timeout_secs);
        let mut count = 0u32;
        let last_message: Option<serde_json::Value> = None;

        while start.elapsed() < timeout {
            if self.pump_once().await? {
                count += 1;
                // Check predicate on last received message
                if let Some(msg) = &last_message {
                    if predicate(msg) {
                        break;
                    }
                }
            } else {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        }

        Ok(count)
    }

    /// Inject a message into the processing pipeline (for testing/simulation)
    pub async fn inject(&self, message: &str) -> Result<()> {
        if let Ok(response) = serde_json::from_str::<WsResponse>(message) {
            if let Some(params) = response.params {
                if let Some(channel) = params.get("channel").and_then(|v| v.as_str()) {
                    let callbacks_read = self.callbacks.read().await;
                    if let Some(callback) = callbacks_read.get(channel) {
                        let callback_clone = Arc::clone(callback);
                        let params_clone = params.clone();
                        tokio::spawn(async move {
                            callback_clone(params_clone).await;
                        });
                    }
                }
            }
        }
        Ok(())
    }
}
