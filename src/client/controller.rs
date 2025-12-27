use super::{Client, Error, ControllerStatus, ControllerNetwork, ControllerNetworkConfig, ControllerMember, ControllerMemberConfig};
use serde_json::json;

/// 控制器管理服务（自托管时可用）
pub struct ControllerService {
    client: Client,
}

impl ControllerService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// 获取控制器状态
    pub async fn status(&self) -> Result<ControllerStatus, Error> {
        self.client.request(reqwest::Method::GET, "/controller", None::<&()>).await
    }

    /// 列出控制器管理的所有网络
    pub async fn list_networks(&self) -> Result<Vec<String>, Error> {
        self.client.request(reqwest::Method::GET, "/controller/network", None::<&()>).await
    }

    /// 获取网络配置
    pub async fn get_network(&self, network_id: &str) -> Result<ControllerNetwork, Error> {
        self.client.request(reqwest::Method::GET, &format!("/controller/network/{}", network_id), None::<&()>).await
    }

    /// 创建新网络（network_id 格式：nodeID + 6个下划线）
    pub async fn create_network(&self, node_id: &str, config: Option<&ControllerNetworkConfig>) -> Result<ControllerNetwork, Error> {
        let network_id = format!("{}______", node_id);
        let body = config.map(|c| serde_json::to_value(c).unwrap()).unwrap_or(json!({}));
        self.client.request(reqwest::Method::POST, &format!("/controller/network/{}", network_id), Some(&body)).await
    }

    /// 更新网络配置
    pub async fn update_network(&self, network_id: &str, config: &ControllerNetworkConfig) -> Result<ControllerNetwork, Error> {
        self.client.request(reqwest::Method::POST, &format!("/controller/network/{}", network_id), Some(config)).await
    }

    /// 删除网络
    pub async fn delete_network(&self, network_id: &str) -> Result<(), Error> {
        self.client.request_empty(reqwest::Method::DELETE, &format!("/controller/network/{}", network_id)).await
    }

    /// 列出网络成员
    pub async fn list_members(&self, network_id: &str) -> Result<Vec<String>, Error> {
        self.client.request(reqwest::Method::GET, &format!("/controller/network/{}/member", network_id), None::<&()>).await
    }

    /// 获取成员信息
    pub async fn get_member(&self, network_id: &str, member_id: &str) -> Result<ControllerMember, Error> {
        self.client.request(reqwest::Method::GET, &format!("/controller/network/{}/member/{}", network_id, member_id), None::<&()>).await
    }

    /// 更新成员配置
    pub async fn update_member(&self, network_id: &str, member_id: &str, config: &ControllerMemberConfig) -> Result<ControllerMember, Error> {
        self.client.request(reqwest::Method::POST, &format!("/controller/network/{}/member/{}", network_id, member_id), Some(config)).await
    }

    /// 删除成员
    pub async fn delete_member(&self, network_id: &str, member_id: &str) -> Result<(), Error> {
        self.client.request_empty(reqwest::Method::DELETE, &format!("/controller/network/{}/member/{}", network_id, member_id)).await
    }
}
