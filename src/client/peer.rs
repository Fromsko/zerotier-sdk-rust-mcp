use super::{Client, Error, Peer};

/// 节点管理服务
pub struct PeerService {
    client: Client,
}

impl PeerService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// 列出所有 Peers
    pub async fn list(&self) -> Result<Vec<Peer>, Error> {
        self.client.request(reqwest::Method::GET, "/peer", None::<&()>).await
    }

    /// 获取指定 Peer 信息
    pub async fn get(&self, peer_id: &str) -> Result<Peer, Error> {
        self.client.request(reqwest::Method::GET, &format!("/peer/{}", peer_id), None::<&()>).await
    }
}
