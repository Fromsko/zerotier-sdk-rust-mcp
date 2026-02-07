use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// Central API 状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CentralStatus {
    #[serde(default, deserialize_with = "null_default")]
    pub id: String,
    #[serde(rename = "type", default, deserialize_with = "null_default")]
    pub status_type: String,
    pub clock: i64,
    #[serde(default, deserialize_with = "null_default")]
    pub version: String,
    #[serde(default, deserialize_with = "null_default")]
    pub api_version: String,
    pub uptime: i64,
    pub user: Option<StatusUser>,
    #[serde(default)]
    pub read_only_mode: bool,
    #[serde(default)]
    pub login_methods: HashMap<String, bool>,
}

/// 状态中的用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusUser {
    #[serde(default, deserialize_with = "null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "null_default")]
    pub org_id: String,
    #[serde(default, deserialize_with = "null_default")]
    pub display_name: String,
    #[serde(default, deserialize_with = "null_default")]
    pub sms_number: String,
}

/// 网络信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    #[serde(default, deserialize_with = "null_default")]
    pub id: String,
    pub clock: i64,
    pub config: Option<NetworkConfig>,
    #[serde(default, deserialize_with = "null_default")]
    pub description: String,
    #[serde(default, deserialize_with = "null_default")]
    pub rules_source: String,
    #[serde(default, deserialize_with = "null_default")]
    pub owner_id: String,
    pub online_member_count: i32,
    pub authorized_member_count: i32,
    pub total_member_count: i32,
    #[serde(default, deserialize_with = "null_default")]
    pub capabilities_by_name: HashMap<String, i32>,
    #[serde(default, deserialize_with = "null_default")]
    pub tags_by_name: HashMap<String, i32>,
}

/// 网络配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfig {
    #[serde(default, deserialize_with = "null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "null_default")]
    pub name: String,
    pub private: bool,
    pub creation_time: i64,
    pub last_modified: i64,
    pub enable_broadcast: bool,
    pub mtu: i32,
    pub multicast_limit: i32,
    #[serde(default, deserialize_with = "null_default")]
    pub routes: Vec<Route>,
    #[serde(default, deserialize_with = "null_default")]
    pub ip_assignment_pools: Vec<IpAssignmentPool>,
    pub v4_assign_mode: Option<AssignMode>,
    pub v6_assign_mode: Option<AssignMode>,
    pub dns: Option<Dns>,
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
    #[serde(default)]
    pub rfc4193: bool,
    #[serde(rename = "6plane", default)]
    pub n6plane: bool,
}

/// DNS 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dns {
    pub domain: String,
    #[serde(default, deserialize_with = "null_default")]
    pub servers: Vec<String>,
}

/// 网络成员
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    #[serde(default, deserialize_with = "null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "null_default")]
    pub network_id: String,
    #[serde(default, deserialize_with = "null_default")]
    pub node_id: String,
    #[serde(default, deserialize_with = "null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "null_default")]
    pub description: String,
    pub config: Option<MemberConfig>,
    pub last_online: i64,
    pub last_seen: i64,
    #[serde(default, deserialize_with = "null_default")]
    pub physical_address: String,
    #[serde(default, deserialize_with = "null_default")]
    pub client_version: String,
    pub protocol_version: i32,
    pub supports_rules_engine: bool,
}

/// 成员配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberConfig {
    #[serde(default, deserialize_with = "null_default")]
    pub authorized: bool,
    #[serde(default, deserialize_with = "null_default")]
    pub active_bridge: bool,
    #[serde(default, deserialize_with = "null_default")]
    pub no_auto_assign_ips: bool,
    pub creation_time: i64,
    #[serde(default, deserialize_with = "null_default")]
    pub ip_assignments: Vec<String>,
    #[serde(default, deserialize_with = "null_default")]
    pub sso_exempt: bool,
}

/// 创建网络请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNetworkRequest {
    pub config: Option<CreateNetworkConfig>,
}

/// 创建网络配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNetworkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "null_default_opt")]
    pub routes: Option<Vec<Route>>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "null_default_opt")]
    pub ip_assignment_pools: Option<Vec<IpAssignmentPool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v4_assign_mode: Option<AssignMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v6_assign_mode: Option<AssignMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Dns>,
}

/// 更新成员请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<UpdateMemberConfig>,
}

/// 更新成员配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMemberConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_bridge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_auto_assign_ips: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "null_default_opt")]
    pub ip_assignments: Option<Vec<String>>,
}

fn null_default<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    Option::<T>::deserialize(deserializer).map(|opt| opt.unwrap_or_default())
}

fn null_default_opt<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Option::<Option<T>>::deserialize(deserializer).map(|opt| opt.flatten())
}
