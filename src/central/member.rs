use super::{Central, Error, Member, UpdateMemberRequest, UpdateMemberConfig};

/// 成员管理服务
pub struct MemberService {
    client: Central,
    network_id: String,
}

impl MemberService {
    pub(crate) fn new(client: Central, network_id: String) -> Self {
        Self { client, network_id }
    }

    fn base_path(&self) -> String {
        format!("/network/{}/member", self.network_id)
    }

    /// 列出网络所有成员
    pub async fn list(&self) -> Result<Vec<Member>, Error> {
        self.client.request(reqwest::Method::GET, &self.base_path(), None::<&()>).await
    }

    /// 获取成员详情
    pub async fn get(&self, member_id: &str) -> Result<Member, Error> {
        self.client.request(reqwest::Method::GET, &format!("{}/{}", self.base_path(), member_id), None::<&()>).await
    }

    /// 更新成员配置
    pub async fn update(&self, member_id: &str, req: &UpdateMemberRequest) -> Result<Member, Error> {
        self.client.request(reqwest::Method::POST, &format!("{}/{}", self.base_path(), member_id), Some(req)).await
    }

    /// 授权成员
    pub async fn authorize(&self, member_id: &str) -> Result<Member, Error> {
        self.update(member_id, &UpdateMemberRequest {
            config: Some(UpdateMemberConfig {
                authorized: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        }).await
    }

    /// 取消授权
    pub async fn deauthorize(&self, member_id: &str) -> Result<Member, Error> {
        self.update(member_id, &UpdateMemberRequest {
            config: Some(UpdateMemberConfig {
                authorized: Some(false),
                ..Default::default()
            }),
            ..Default::default()
        }).await
    }

    /// 删除成员
    pub async fn delete(&self, member_id: &str) -> Result<(), Error> {
        self.client.request_empty(reqwest::Method::DELETE, &format!("{}/{}", self.base_path(), member_id)).await
    }
}

/// 成员配置构建器
#[derive(Default)]
#[allow(dead_code)]
pub struct MemberConfigBuilder {
    config: UpdateMemberConfig,
}

#[allow(dead_code)]
impl MemberConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authorized(mut self, v: bool) -> Self {
        self.config.authorized = Some(v);
        self
    }

    pub fn active_bridge(mut self, v: bool) -> Self {
        self.config.active_bridge = Some(v);
        self
    }

    pub fn ip_assignments(mut self, ips: Vec<String>) -> Self {
        self.config.ip_assignments = Some(ips);
        self
    }

    pub fn build(self) -> UpdateMemberConfig {
        self.config
    }
}
