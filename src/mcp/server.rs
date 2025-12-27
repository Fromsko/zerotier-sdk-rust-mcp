use crate::{client::Client, central::Central};
use rmcp::{
    ServerHandler,
    model::ServerInfo,
    schemars::{self, JsonSchema},
    tool,
    ServiceExt,
};
use serde::Deserialize;

/// ZeroTier MCP 服务
#[derive(Clone)]
pub struct McpServer {
    local_client: Client,
    central_client: Option<Central>,
}

impl McpServer {
    /// 创建 MCP 服务
    pub fn new() -> Self {
        Self {
            local_client: Client::new(),
            central_client: None,
        }
    }

    /// 设置本地客户端
    pub fn with_local_client(mut self, client: Client) -> Self {
        self.local_client = client;
        self
    }

    /// 设置云端客户端
    pub fn with_central_client(mut self, client: Central) -> Self {
        self.central_client = Some(client);
        self
    }

    /// 使用 Token 设置云端客户端
    pub fn with_central_token(mut self, token: impl Into<String>) -> Self {
        self.central_client = Some(Central::new(token));
        self
    }

    /// 启动 stdio 服务
    pub async fn serve_stdio(self) -> Result<(), Box<dyn std::error::Error>> {
        use rmcp::transport::stdio;
        let transport = stdio();
        let server = self.serve(transport).await?;
        server.waiting().await?;
        Ok(())
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// 工具参数定义
// ============================================

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NetworkIdParam {
    /// 网络 ID（16位十六进制）
    pub network_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MemberParam {
    /// 网络 ID
    pub network_id: String,
    /// 成员 ID
    pub member_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AuthorizeWithIpParam {
    /// 网络 ID
    pub network_id: String,
    /// 成员 ID
    pub member_id: String,
    /// 自定义 IP 地址（如 "10.147.20.100"）
    pub ip_address: String,
}

// ============================================
// 工具实现
// ============================================

#[tool(tool_box)]
impl McpServer {
    /// 获取本地 ZeroTier 节点状态
    #[tool(description = "获取本地 ZeroTier 节点状态")]
    async fn zt_status(&self) -> String {
        match self.local_client.status().await {
            Ok(status) => {
                format!(
                    "节点状态:\n- 地址: {}\n- 版本: {}\n- 在线: {}\n- TCP回退: {}",
                    status.address, status.version, status.online, status.tcp_fallback_active
                )
            }
            Err(e) => format!("获取状态失败: {}", e),
        }
    }

    /// 列出已加入的 ZeroTier 网络
    #[tool(description = "列出已加入的 ZeroTier 网络")]
    async fn zt_networks(&self) -> String {
        match self.local_client.networks().list().await {
            Ok(networks) => {
                if networks.is_empty() {
                    return "暂未加入任何网络".to_string();
                }
                let mut result = String::from("已加入的网络:\n");
                for n in networks {
                    result.push_str(&format!("\n[{}] {}\n", n.id, n.name));
                    result.push_str(&format!("  状态: {}\n", n.status));
                    result.push_str(&format!("  IP: {:?}\n", n.assigned_addresses));
                }
                result
            }
            Err(e) => format!("获取网络失败: {}", e),
        }
    }

    /// 加入 ZeroTier 网络
    #[tool(description = "加入 ZeroTier 网络")]
    async fn zt_join(&self, #[tool(aggr)] param: NetworkIdParam) -> String {
        match self.local_client.networks().join(&param.network_id).await {
            Ok(network) => format!("已加入网络: {} ({})", network.id, network.name),
            Err(e) => format!("加入网络失败: {}", e),
        }
    }

    /// 离开 ZeroTier 网络
    #[tool(description = "离开 ZeroTier 网络")]
    async fn zt_leave(&self, #[tool(aggr)] param: NetworkIdParam) -> String {
        match self.local_client.networks().leave(&param.network_id).await {
            Ok(()) => format!("已离开网络: {}", param.network_id),
            Err(e) => format!("离开网络失败: {}", e),
        }
    }

    /// 列出所有 ZeroTier Peers
    #[tool(description = "列出所有 ZeroTier Peers")]
    async fn zt_peers(&self) -> String {
        match self.local_client.peers().list().await {
            Ok(peers) => {
                if peers.is_empty() {
                    return "暂无 Peers".to_string();
                }
                let mut result = String::from("Peers:\n");
                for p in peers {
                    result.push_str(&format!("\n[{}]\n", p.address));
                    result.push_str(&format!("  角色: {}\n", p.role));
                    result.push_str(&format!("  版本: {}\n", p.version));
                    result.push_str(&format!("  延迟: {}ms\n", p.latency));
                }
                result
            }
            Err(e) => format!("获取 Peers 失败: {}", e),
        }
    }

    /// 列出云端 ZeroTier 网络
    #[tool(description = "列出云端 ZeroTier 网络")]
    async fn zt_central_networks(&self) -> String {
        let Some(ref client) = self.central_client else {
            return "未配置 Central API Token".to_string();
        };

        match client.networks().list().await {
            Ok(networks) => {
                if networks.is_empty() {
                    return "暂无网络".to_string();
                }
                let mut result = String::from("云端网络:\n");
                for n in networks {
                    let name = n.config.as_ref().map(|c| c.name.as_str()).unwrap_or("未命名");
                    result.push_str(&format!("\n[{}] {}\n", n.id, name));
                    result.push_str(&format!(
                        "  在线: {} / 授权: {} / 总计: {}\n",
                        n.online_member_count, n.authorized_member_count, n.total_member_count
                    ));
                }
                result
            }
            Err(e) => format!("获取网络失败: {}", e),
        }
    }

    /// 列出网络成员
    #[tool(description = "列出网络成员")]
    async fn zt_central_members(&self, #[tool(aggr)] param: NetworkIdParam) -> String {
        let Some(ref client) = self.central_client else {
            return "未配置 Central API Token".to_string();
        };

        match client.networks().members(&param.network_id).list().await {
            Ok(members) => {
                if members.is_empty() {
                    return "暂无成员".to_string();
                }
                let mut result = format!("网络 {} 的成员:\n", param.network_id);
                for m in members {
                    let status = if m.config.as_ref().map(|c| c.authorized).unwrap_or(false) {
                        "✅"
                    } else {
                        "❌"
                    };
                    result.push_str(&format!("\n{} [{}] {}\n", status, m.node_id, m.name));
                    if let Some(config) = &m.config {
                        result.push_str(&format!("  IP: {:?}\n", config.ip_assignments));
                    }
                }
                result
            }
            Err(e) => format!("获取成员失败: {}", e),
        }
    }

    /// 授权网络成员
    #[tool(description = "授权网络成员")]
    async fn zt_central_authorize(&self, #[tool(aggr)] param: MemberParam) -> String {
        let Some(ref client) = self.central_client else {
            return "未配置 Central API Token".to_string();
        };

        match client.networks().members(&param.network_id).authorize(&param.member_id).await {
            Ok(member) => format!("已授权成员: {} ({})", member.node_id, member.name),
            Err(e) => format!("授权失败: {}", e),
        }
    }

    /// 授权网络成员并指定 IP 地址
    #[tool(description = "授权网络成员并指定自定义 IP 地址")]
    async fn zt_central_authorize_with_ip(&self, #[tool(aggr)] param: AuthorizeWithIpParam) -> String {
        use crate::central::{UpdateMemberRequest, UpdateMemberConfig};
        
        let Some(ref client) = self.central_client else {
            return "未配置 Central API Token".to_string();
        };

        let req = UpdateMemberRequest {
            config: Some(UpdateMemberConfig {
                authorized: Some(true),
                ip_assignments: Some(vec![param.ip_address.clone()]),
                ..Default::default()
            }),
            ..Default::default()
        };

        match client.networks().members(&param.network_id).update(&param.member_id, &req).await {
            Ok(member) => {
                let ips = member.config.as_ref()
                    .map(|c| c.ip_assignments.join(", "))
                    .unwrap_or_default();
                format!("已授权成员: {} ({})\nIP: {}", member.node_id, member.name, ips)
            }
            Err(e) => format!("授权失败: {}", e),
        }
    }

    /// 取消成员授权
    #[tool(description = "取消成员授权")]
    async fn zt_central_deauthorize(&self, #[tool(aggr)] param: MemberParam) -> String {
        let Some(ref client) = self.central_client else {
            return "未配置 Central API Token".to_string();
        };

        match client.networks().members(&param.network_id).deauthorize(&param.member_id).await {
            Ok(member) => format!("已取消授权: {} ({})", member.node_id, member.name),
            Err(e) => format!("取消授权失败: {}", e),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("ZeroTier SDK MCP Server - 管理本地和云端 ZeroTier 网络".into()),
            ..Default::default()
        }
    }
}
