# ZeroTier Central null 响应问题排查总结

## 问题概述
- 调用云端网络列表时，API 返回 2xx，但 JSON 解析报错：`invalid type: null, expected a sequence`。
- 响应中部分字段会返回 `null`，而模型结构使用了必填 Vec/Map，导致反序列化失败。

## 修复点
- 放宽反序列化，允许 `null` -> 空集合：
  - `NetworkConfig.routes`、`ip_assignment_pools`
  - `MemberConfig.ip_assignments`
  - `UpdateMemberConfig.ip_assignments`
  - `Network.capabilities_by_name`、`tags_by_name`
  - `Dns.servers`
- `CreateNetworkConfig` 中的可选 Vec 字段也接受 `null`（转为空或 None）。
- 保留空体与非 JSON 响应的健壮性检查（之前已加）。

## 代码位置
- `src/central/types.rs`
- `src/central/mod.rs`（健壮性检查：空响应/非 JSON 直接报错）

## 现状
- 重新构建并部署到 `C:\coding\envs\mcps\zerotier-mcp.exe`。
- 云端网络列表已可正常返回，例如：`[45b6e887e2bb5bd9] Fromsko`。

## 使用提示
- 确保 `ZEROTIER_CENTRAL_TOKEN` 已设置后再启动 MCP 服务。
- 若再遇解析问题，可打印 Content-Type 与原始 body 片段以辅助定位。
