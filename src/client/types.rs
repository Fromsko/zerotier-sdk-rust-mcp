use serde::{Deserialize, Serialize};

/// 节点状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    pub address: String,
    pub clock: i64,
    pub online: bool,
    pub planet_world_id: i64,
    pub public_identity: String,
    #[serde(rename = "tcpFallbackActive")]
    pub tcp_fallback_active: bool,
    pub version: String,
}

/// 本地网络信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(rename = "type")]
    pub network_type: String,
    pub mac: String,
    pub mtu: i32,
    pub bridge: bool,
    pub broadcast_enabled: bool,
    pub port_device_name: String,
    pub netconf_revision: i32,
    pub assigned_addresses: Vec<String>,
    pub allow_dns: bool,
    pub allow_default: bool,
    pub allow_global: bool,
    pub allow_managed: bool,
    pub dns: Option<Dns>,
}

/// DNS 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dns {
    pub domain: String,
    #[serde(default)]
    pub servers: Vec<String>,
}

/// 网络设置（用于更新）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_dns: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_global: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_managed: Option<bool>,
}

/// 节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub address: String,
    pub version: String,
    pub role: String,
    pub latency: i32,
    pub paths: Vec<PeerPath>,
}

/// 节点路径
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerPath {
    pub active: bool,
    pub address: String,
    pub expired: bool,
    pub last_receive: i64,
    pub last_send: i64,
    pub preferred: bool,
    pub trusted_path_id: i64,
}

/// 控制器状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatus {
    pub controller: bool,
    pub api_version: i32,
    pub clock: i64,
}

/// 控制器网络配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerNetwork {
    pub id: String,
    pub name: String,
    pub private: bool,
    pub creation_time: i64,
    pub revision: i32,
    pub multicast_limit: i32,
    pub enable_broadcast: bool,
    #[serde(default)]
    pub routes: Vec<Route>,
    #[serde(default)]
    pub ip_assignment_pools: Vec<IpAssignmentPool>,
    pub v4_assign_mode: Option<AssignMode>,
    pub v6_assign_mode: Option<AssignMode>,
}

/// 路由配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
}

/// IP 分配池
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpAssignmentPool {
    pub ip_range_start: String,
    pub ip_range_end: String,
}

/// IP 分配模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignMode {
    pub zt: bool,
}

/// 控制器成员
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerMember {
    pub id: String,
    pub address: String,
    pub network_id: String,
    pub authorized: bool,
    pub active_bridge: bool,
    #[serde(default)]
    pub ip_assignments: Vec<String>,
    pub no_auto_assign_ips: bool,
    pub revision: i32,
    pub creation_time: i64,
    pub last_authorized_time: i64,
    pub last_deauthorized_time: i64,
}

/// 创建/更新网络的配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerNetworkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<Route>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_assignment_pools: Option<Vec<IpAssignmentPool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_assign_mode: Option<AssignMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v6_assign_mode: Option<AssignMode>,
}

/// 成员配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerMemberConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_bridge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_assignments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_auto_assign_ips: Option<bool>,
}
