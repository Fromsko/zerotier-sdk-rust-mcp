//! ZeroTier Central API（云端）客户端

mod types;
mod network;
mod member;

pub use types::*;
pub use network::NetworkService;
pub use member::MemberService;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::time::Duration;

/// ZeroTier Central API 客户端
#[derive(Clone)]
pub struct Central {
    base_url: String,
    token: String,
    http_client: reqwest::Client,
}

/// 客户端配置选项
#[derive(Default)]
pub struct CentralOptions {
    pub base_url: Option<String>,
    pub timeout: Option<Duration>,
}

impl Central {
    /// 创建新的 Central API 客户端
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_options(token, CentralOptions::default())
    }

    /// 使用配置选项创建客户端
    pub fn with_options(token: impl Into<String>, opts: CentralOptions) -> Self {
        let base_url = opts.base_url.unwrap_or_else(|| "https://api.zerotier.com/api/v1".to_string());
        let timeout = opts.timeout.unwrap_or(Duration::from_secs(30));
        
        let http_client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url,
            token: token.into(),
            http_client,
        }
    }

    /// 执行 HTTP 请求
    pub(crate) async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&impl serde::Serialize>,
    ) -> Result<T, Error> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", self.token)).unwrap());

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
    ) -> Result<(), Error> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", self.token)).unwrap());

        let resp = self.http_client.request(method, &url).headers(headers).send().await?;
        
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::Api { status: status.as_u16(), message: text });
        }

        Ok(())
    }

    /// 获取 Central 状态
    pub async fn status(&self) -> Result<CentralStatus, Error> {
        self.request(reqwest::Method::GET, "/status", None::<&()>).await
    }

    /// 网络管理服务
    pub fn networks(&self) -> NetworkService {
        NetworkService::new(self.clone())
    }
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
