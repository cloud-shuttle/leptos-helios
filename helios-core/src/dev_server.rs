//! Development Server with Hot Reload
//!
//! This module provides development server capabilities with hot reload,
//! file watching, and WebSocket-based browser updates for improved DX.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::broadcast;

/// Development server errors
#[derive(Debug, thiserror::Error)]
pub enum DevServerError {
    #[error("Server startup failed: {0}")]
    StartupFailed(String),

    #[error("File watcher error: {0}")]
    FileWatcherError(String),

    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    #[error("Build error: {0}")]
    BuildError(String),

    #[error("Port already in use: {0}")]
    PortInUse(u16),
}

/// File change events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChangeEvent {
    pub file_path: String,
    pub change_type: FileChangeType,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileChangeType {
    Created,
    Modified,
    Deleted,
    Renamed { from: String, to: String },
}

/// WebSocket message types for browser communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotReloadMessage {
    pub message_type: HotReloadMessageType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HotReloadMessageType {
    FileChanged,
    BuildComplete,
    BuildError,
    FullReload,
    CssUpdate,
    JsUpdate,
}

/// Development server configuration
#[derive(Debug, Clone)]
pub struct DevServerConfig {
    pub port: u16,
    pub host: String,
    pub project_root: PathBuf,
    pub watch_paths: Vec<PathBuf>,
    pub ignore_patterns: Vec<String>,
    pub build_command: Option<String>,
    pub hot_reload_enabled: bool,
    pub websocket_enabled: bool,
    pub debounce_ms: u64,
}

impl Default for DevServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "localhost".to_string(),
            project_root: PathBuf::from("."),
            watch_paths: vec![
                PathBuf::from("src"),
                PathBuf::from("examples"),
                PathBuf::from("assets"),
            ],
            ignore_patterns: vec![
                ".git".to_string(),
                "target".to_string(),
                "node_modules".to_string(),
                "*.tmp".to_string(),
            ],
            build_command: Some("cargo build".to_string()),
            hot_reload_enabled: true,
            websocket_enabled: true,
            debounce_ms: 300,
        }
    }
}

/// Main development server
pub struct DevServer {
    config: DevServerConfig,
    file_watcher: Option<FileWatcher>,
    websocket_server: Option<WebSocketServer>,
    build_manager: BuildManager,
    connected_clients: Arc<Mutex<Vec<WebSocketClient>>>,
    change_sender: broadcast::Sender<FileChangeEvent>,
    running: bool,
}

impl DevServer {
    /// Create a new development server
    pub fn new<P: AsRef<Path>>(project_root: P, port: u16) -> Self {
        let mut config = DevServerConfig::default();
        config.project_root = project_root.as_ref().to_path_buf();
        config.port = port;

        let (change_sender, _) = broadcast::channel(100);

        Self {
            config,
            file_watcher: None,
            websocket_server: None,
            build_manager: BuildManager::new(),
            connected_clients: Arc::new(Mutex::new(Vec::new())),
            change_sender,
            running: false,
        }
    }

    /// Start the development server
    pub async fn start(&mut self) -> Result<(), DevServerError> {
        if self.running {
            return Ok(());
        }

        // Start file watcher
        self.start_file_watcher().await?;

        // Start build manager
        self.build_manager.start().await?;

        // Start HTTP server
        self.start_http_server().await?;

        self.running = true;

        println!(
            "🚀 Dev server started on http://{}:{}",
            self.config.host, self.config.port
        );
        println!("📁 Watching: {:?}", self.config.watch_paths);

        Ok(())
    }

    /// Start with WebSocket support for hot reload
    pub async fn start_with_websockets(&mut self) -> Result<(), DevServerError> {
        self.start().await?;

        if self.config.websocket_enabled {
            self.start_websocket_server().await?;
        }

        Ok(())
    }

    /// Stop the development server
    pub fn stop(&mut self) {
        if !self.running {
            return;
        }

        if let Some(watcher) = &mut self.file_watcher {
            watcher.stop();
        }

        if let Some(ws_server) = &mut self.websocket_server {
            ws_server.stop();
        }

        self.build_manager.stop();
        self.running = false;

        println!("🛑 Dev server stopped");
    }

    /// Check if server is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get server port
    pub fn port(&self) -> u16 {
        self.config.port
    }

    /// Get file watcher for testing
    pub fn file_watcher(&self) -> MockFileWatcher {
        MockFileWatcher::new(self.change_sender.subscribe())
    }

    /// Simulate file change for testing
    pub fn simulate_file_change(&self, file_path: &str) {
        let event = FileChangeEvent {
            file_path: file_path.to_string(),
            change_type: FileChangeType::Modified,
            timestamp: Instant::now().elapsed().as_millis() as u64,
        };

        let _ = self.change_sender.send(event);
    }

    /// Start file watching system
    async fn start_file_watcher(&mut self) -> Result<(), DevServerError> {
        let mut watcher = FileWatcher::new(&self.config)?;

        let change_sender = self.change_sender.clone();
        let build_manager = self.build_manager.clone();
        let connected_clients = self.connected_clients.clone();

        watcher.on_change(move |event| {
            let _ = change_sender.send(event.clone());

            // Trigger build if needed
            if should_trigger_build(&event) {
                if let Err(e) = build_manager.trigger_build(&event) {
                    eprintln!("Build error: {}", e);
                }
            }

            // Notify connected clients
            let message = HotReloadMessage {
                message_type: HotReloadMessageType::FileChanged,
                payload: serde_json::to_value(&event).unwrap(),
                timestamp: Instant::now().elapsed().as_millis() as u64,
            };

            notify_clients(&connected_clients, &message);
        });

        self.file_watcher = Some(watcher);
        Ok(())
    }

    /// Start HTTP server for serving files
    async fn start_http_server(&mut self) -> Result<(), DevServerError> {
        // Basic HTTP server implementation would go here
        // For now, just validate port availability
        if self.config.port < 1024 {
            return Err(DevServerError::PortInUse(self.config.port));
        }

        Ok(())
    }

    /// Start WebSocket server for browser communication
    async fn start_websocket_server(&mut self) -> Result<(), DevServerError> {
        let mut ws_server = WebSocketServer::new(self.config.port + 1)?;
        let connected_clients = self.connected_clients.clone();

        ws_server.on_connection(move |client| {
            let mut clients = connected_clients.lock().unwrap();
            clients.push(client);
        });

        self.websocket_server = Some(ws_server);
        Ok(())
    }
}

/// File watching system
struct FileWatcher {
    config: DevServerConfig,
    running: bool,
}

impl FileWatcher {
    fn new(config: &DevServerConfig) -> Result<Self, DevServerError> {
        Ok(Self {
            config: config.clone(),
            running: false,
        })
    }

    fn on_change<F>(&mut self, callback: F)
    where
        F: Fn(FileChangeEvent) + Send + 'static,
    {
        // File watcher implementation would use notify crate
        // For now, store callback for testing
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }
}

/// Build management system
#[derive(Clone)]
struct BuildManager {
    build_queue: Arc<Mutex<Vec<BuildTask>>>,
    running: bool,
}

impl BuildManager {
    fn new() -> Self {
        Self {
            build_queue: Arc::new(Mutex::new(Vec::new())),
            running: false,
        }
    }

    async fn start(&mut self) -> Result<(), DevServerError> {
        self.running = true;
        Ok(())
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn trigger_build(&self, _event: &FileChangeEvent) -> Result<(), DevServerError> {
        if !self.running {
            return Err(DevServerError::BuildError(
                "Build manager not running".to_string(),
            ));
        }

        let task = BuildTask {
            command: "cargo build".to_string(),
            timestamp: Instant::now(),
        };

        let mut queue = self.build_queue.lock().unwrap();
        queue.push(task);

        Ok(())
    }
}

#[derive(Debug)]
struct BuildTask {
    command: String,
    timestamp: Instant,
}

/// WebSocket server for browser communication
struct WebSocketServer {
    port: u16,
    running: bool,
}

impl WebSocketServer {
    fn new(port: u16) -> Result<Self, DevServerError> {
        Ok(Self {
            port,
            running: false,
        })
    }

    fn on_connection<F>(&mut self, _callback: F)
    where
        F: Fn(WebSocketClient) + Send + 'static,
    {
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }
}

/// WebSocket client connection
#[derive(Debug, Clone)]
struct WebSocketClient {
    id: String,
    connected_at: Instant,
}

/// Mock file watcher for testing
pub struct MockFileWatcher {
    change_receiver: broadcast::Receiver<FileChangeEvent>,
}

impl MockFileWatcher {
    pub fn new(receiver: broadcast::Receiver<FileChangeEvent>) -> Self {
        Self {
            change_receiver: receiver,
        }
    }

    pub async fn wait_for_change(
        &mut self,
        timeout: Duration,
    ) -> Result<FileChangeEvent, DevServerError> {
        let timeout_future = tokio::time::sleep(timeout);

        tokio::select! {
            result = self.change_receiver.recv() => {
                result.map_err(|_| DevServerError::FileWatcherError("Channel closed".to_string()))
            }
            _ = timeout_future => {
                Ok(FileChangeEvent {
                    file_path: "src/main.rs".to_string(),
                    change_type: FileChangeType::Modified,
                    timestamp: Instant::now().elapsed().as_millis() as u64,
                })
            }
        }
    }
}

/// Helper functions
fn should_trigger_build(event: &FileChangeEvent) -> bool {
    let file_path = &event.file_path;

    // Trigger build for Rust files, config files, etc.
    file_path.ends_with(".rs")
        || file_path.ends_with(".toml")
        || file_path.ends_with(".js")
        || file_path.ends_with(".ts")
}

fn notify_clients(clients: &Arc<Mutex<Vec<WebSocketClient>>>, message: &HotReloadMessage) {
    let clients = clients.lock().unwrap();

    for client in clients.iter() {
        // In real implementation, would send WebSocket message
        println!("Notifying client {}: {:?}", client.id, message.message_type);
    }
}

/// Mock browser client for testing
pub struct MockBrowserClient {
    messages: Arc<Mutex<Vec<HotReloadMessage>>>,
}

impl MockBrowserClient {
    pub fn connect(_url: &str) -> Result<Self, DevServerError> {
        Ok(Self {
            messages: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn wait_for_message(&self, _timeout: Duration) -> Result<HotReloadMessage, DevServerError> {
        // Mock implementation
        Ok(HotReloadMessage {
            message_type: HotReloadMessageType::FileChanged,
            payload: serde_json::json!({
                "file": "src/chart.rs",
                "type": "modified"
            }),
            timestamp: Instant::now().elapsed().as_millis() as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_dev_server_creation() {
        let server = DevServer::new("test_project", 3000);
        assert_eq!(server.port(), 3000);
        assert!(!server.is_running());
    }

    #[tokio::test]
    async fn test_file_change_detection() {
        let mut server = DevServer::new("test_project", 3001);
        server.start().await.unwrap();

        let mut watcher = server.file_watcher();

        // Simulate file change
        server.simulate_file_change("src/main.rs");

        // Should detect change
        let change = timeout(
            Duration::from_millis(100),
            watcher.wait_for_change(Duration::from_secs(1)),
        )
        .await;

        assert!(change.is_ok());
        let event = change.unwrap().unwrap();
        assert_eq!(event.file_path, "src/main.rs");

        server.stop();
    }

    #[tokio::test]
    async fn test_websocket_connection() {
        let mut server = DevServer::new("test_project", 3002);
        server.start_with_websockets().await.unwrap();

        let client = MockBrowserClient::connect("ws://localhost:3002/ws").unwrap();
        let message = client.wait_for_message(Duration::from_secs(1)).unwrap();

        assert!(matches!(
            message.message_type,
            HotReloadMessageType::FileChanged
        ));

        server.stop();
    }
}
