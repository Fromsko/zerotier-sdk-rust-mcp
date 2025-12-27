use super::{Central, Error, Network, CreateNetworkConfig, CreateNetworkRequest, MemberService};

/// 网络管理服务
pub struct NetworkService {
    client: Central,
}

impl NetworkService {
    pub(crate) fn new(client: Central) -> Self {
        Self { client }
    }

    /// 列出所有网络
    pub async fn list(&self) -> Result<Vec<Network>, Error> {
        self.client.request(reqwest::Method::GET, "/network", None::<&()>).await
    }

    /// 获取网络详情
    pub async fn get(&self, network_id: &str) -> Result<Network, Error> {
        self.client.request(reqwest::Method::GET, &format!("/network/{}", network_id), None::<&()>).await
    }

    /// 创建新网络
    pub async fn create(&self, config: Option<&CreateNetworkConfig>) -> Result<Network, Error> {
        let req = CreateNetworkRequest { config: config.cloned() };
        self.client.request(reqwest::Method::POST, "/network", Some(&req)).await
    }

    /// 更新网络配置
    pub async fn update(&self, network_id: &str, config: &CreateNetworkConfig) -> Result<Network, Error> {
        let req = CreateNetworkRequest { config: Some(config.clone()) };
        self.client.request(reqwest::Method::POST, &format!("/network/{}", network_id), Some(&req)).await
    }

    /// 删除网络
    pub async fn delete(&self, network_id: &str) -> Result<(), Error> {
        self.client.request_empty(reqwest::Method::DELETE, &format!("/network/{}", network_id)).await
    }

    /// 获取成员服务
    pub fn members(&self, network_id: &str) -> MemberService {
        MemberService::new(self.client.clone(), network_id.to_string())
    }
}

/// 网络配置构建器
#[derive(Default)]
#[allow(dead_code)]
pub struct NetworkConfigBuilder {
    config: CreateNetworkConfig,
}

#[allow(dead_code)]
impl NetworkConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.config.name = Some(name.into());
        self
    }

    pub fn private(mut self, v: bool) -> Self {
        self.config.private = Some(v);
        self
    }

    pub fn enable_broadcast(mut self, v: bool) -> Self {
        self.config.enable_broadcast = Some(v);
        self
    }

    pub fn mtu(mut self, v: i32) -> Self {
        self.config.mtu = Some(v);
        self
    }

    pub fn build(self) -> CreateNetworkConfig {
        self.config
    }
}
