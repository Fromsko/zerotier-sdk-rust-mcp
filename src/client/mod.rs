//! ZeroTier Service API（本地节点）客户端

mod types;
mod network;
mod peer;
mod controller;

pub use types::*;
pub use network::{NetworkService, NetworkSettingsBuilder};
pub use peer::PeerService;
pub use controller::ControllerService;

use reqwest::header::{HeaderMap, HeaderValue};
use std::time::Duration;

/// ZeroTier Service API 客户端
#[derive(Clone)]
pub struct Client {
    base_url: String,
    token: String,
    http_client: reqwest::Client,
}

/// 客户端配置选项
#[derive(Default)]
pub struct ClientOptions {
    pub base_url: Option<String>,
    pub token: Option<String>,
    pub token_file: Option<String>,
    pub timeout: Option<Duration>,
}

impl Client {
    /// 创建新的 ZeroTier Service API 客户端
    pub fn new() -> Self {
        Self::with_options(ClientOptions::default())
    }

    /// 使用配置选项创建客户端
    pub fn with_options(opts: ClientOptions) -> Self {
        let base_url = opts.base_url.unwrap_or_else(|| "http://localhost:9993".to_string());
        
        let token = opts.token
            .or_else(|| opts.token_file.and_then(|p| std::fs::read_to_string(p).ok()))
            .or_else(read_default_token)
            .unwrap_or_default()
            .trim()
            .to_string();

        let timeout = opts.timeout.unwrap_or(Duration::from_secs(10));
        
        let http_client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self { base_url, token, http_client }
    }

    /// 使用 Token 创建客户端
    pub fn with_token(token: impl Into<String>) -> Self {
        Self::with_options(ClientOptions {
            token: Some(token.into()),
            ..Default::default()
        })
    }

    /// 执行 HTTP 请求
    pub(crate) async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&impl serde::Serialize>,
    ) -> Result<T, crate::client::Error> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut headers = HeaderMap::new();
        headers.insert("X-ZT1-AUTH", HeaderValue::from_str(&self.token).unwrap());

        let mut req = self.http_client.request(method, &url).headers(headers);
        
        if let Some(b) = body {
            req = req.json(b);
        }

        let resp = req.send().await?;
        
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: text });
        }

        Ok(resp.json().await?)
    }

    /// 执行无返回值的 HTTP 请求
    pub(crate) async fn request_empty(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> Result<(), crate::client::Error> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut headers = HeaderMap::new();
        headers.insert("X-ZT1-AUTH", HeaderValue::from_str(&self.token).unwrap());

        let resp = self.http_client.request(method, &url).headers(headers).send().await?;
        
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: text });
        }

        Ok(())
    }

    /// 获取节点状态
    pub async fn status(&self) -> Result<NodeStatus, Error> {
        self.request(reqwest::Method::GET, "/status", None::<&()>).await
    }

    /// 网络管理服务
    pub fn networks(&self) -> NetworkService {
        NetworkService::new(self.clone())
    }

    /// 节点管理服务
    pub fn peers(&self) -> PeerService {
        PeerService::new(self.clone())
    }

    /// 控制器管理服务
    pub fn controller(&self) -> ControllerService {
        ControllerService::new(self.clone())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// 从系统默认位置读取 token
fn read_default_token() -> Option<String> {
    let path = if cfg!(windows) {
        r"C:\ProgramData\ZeroTier\One\authtoken.secret".to_string()
    } else if cfg!(target_os = "macos") {
        dirs::home_dir()
            .map(|h| h.join("Library/Application Support/ZeroTier/authtoken.secret"))
            .map(|p| p.to_string_lossy().to_string())?
    } else {
        "/var/lib/zerotier-one/authtoken.secret".to_string()
    };

    std::fs::read_to_string(path).ok()
}

/// 错误类型
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
