use super::{Client, Error, Network, NetworkSettings};
use serde_json::json;

/// 网络管理服务
pub struct NetworkService {
    client: Client,
}

impl NetworkService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// 列出所有已加入的网络
    pub async fn list(&self) -> Result<Vec<Network>, Error> {
        self.client.request(reqwest::Method::GET, "/network", None::<&()>).await
    }

    /// 获取指定网络详情
    pub async fn get(&self, network_id: &str) -> Result<Network, Error> {
        self.client.request(reqwest::Method::GET, &format!("/network/{}", network_id), None::<&()>).await
    }

    /// 加入网络
    pub async fn join(&self, network_id: &str) -> Result<Network, Error> {
        self.client.request(reqwest::Method::POST, &format!("/network/{}", network_id), Some(&json!({}))).await
    }

    /// 离开网络
    pub async fn leave(&self, network_id: &str) -> Result<(), Error> {
        self.client.request_empty(reqwest::Method::DELETE, &format!("/network/{}", network_id)).await
    }

    /// 更新网络设置
    pub async fn update(&self, network_id: &str, settings: &NetworkSettings) -> Result<Network, Error> {
        self.client.request(reqwest::Method::POST, &format!("/network/{}", network_id), Some(settings)).await
    }
}

/// 网络设置构建器
#[derive(Default)]
pub struct NetworkSettingsBuilder {
    settings: NetworkSettings,
}

impl NetworkSettingsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_dns(mut self, v: bool) -> Self {
        self.settings.allow_dns = Some(v);
        self
    }

    pub fn allow_default(mut self, v: bool) -> Self {
        self.settings.allow_default = Some(v);
        self
    }

    pub fn allow_global(mut self, v: bool) -> Self {
        self.settings.allow_global = Some(v);
        self
    }

    pub fn allow_managed(mut self, v: bool) -> Self {
        self.settings.allow_managed = Some(v);
        self
    }

    pub fn build(self) -> NetworkSettings {
        self.settings
    }
}
