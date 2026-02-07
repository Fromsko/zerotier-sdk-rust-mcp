# 项目设计与实践速览（zerotier-sdk-rust-mcp）

## 架构与模块
- **层次**：`client`（本地 ZeroTier Service API） / `central`（云端 Central API） / `mcp`（工具服务层） / `bin`（启动入口）。
- **lib 暴露**：`Client`、`Central`、`McpServer` 作为主要对外类型。
- **MCP 工具**：基于 `rmcp` 的工具路由，提供本地/云端的查询、加入/退出网络、授权等操作。

## 设计风格
- **简单直观的服务对象**：`Client`, `Central`, `NetworkService`, `MemberService` 以面向资源的服务划分。
- **轻量错误模型**：单一 `Error` 枚举，区分 HTTP、API、自定义 JSON 错误；调用端统一 `Result`。
- **串联式构建器**：`NetworkConfigBuilder`、`MemberConfigBuilder` 用链式方法组装请求。
- **环境驱动配置**：入口读取 `ZEROTIER_LOCAL_TOKEN`、`ZEROTIER_CENTRAL_TOKEN`，可缺省/自动读取。
- **健壮性优先**：中央请求层检查空响应、Content-Type 非 JSON 时返回原文错误。
- **容忍松散 JSON**：`null_default` / `null_default_opt` 允许 Central 返回 `null` 时自动转空集合/None。

## 代码编写风格
- 模块化：每个子域独立 `mod.rs` + 资源文件（如 `network.rs`, `member.rs`）。
- Serde 配置：统一 `camelCase`，必填标量直接解析，集合/可选用 `#[serde(default)]` 并在需要时自定义反序列化。
- 简洁入口：`src/bin/zerotier-mcp.rs` 只做 token 读取与 `McpServer` 装配。
- 最小注释，命名直白：`status`, `networks().list()`, `members().authorize()`。

## 设计模式/实践
- **Service per resource**：网络、成员、Peers 等分别暴露 service，便于扩展。
- **Builder for mutable payload**：避免直接构造大 struct，链式填可选字段。
- **Result early-return**：HTTP 层失败早返回 `Error::Api`，避免深层 unwrap。
- **Token injection**：可用 `with_central_token` / `with_local_client` 注入，方便测试与自定义。

## MCP 初始化与使用
- 配置：在 MCP 客户端配置中指向 `zerotier-mcp` 可执行，并提供环境变量。
- 入口流程：
  1. 读取环境变量 token（本地可缺省自动读取系统 authtoken.secret）。
  2. 构造 `Client`（本地）和可选 `Central`。
  3. `McpServer::new().with_local_client(...).with_central_token(...)` 后 `serve_stdio()`。
- 工具能力（示例）：
  - 本地：`zt_status`、`zt_networks`、`zt_join`、`zt_leave`、`zt_peers`。
  - 云端：`zt_central_networks`、成员授权/去授权等（通过 `MemberService`）。

## 最佳实践
- **必备环境**：确保 `ZEROTIER_CENTRAL_TOKEN` 在启动前已配置；本地 Token 可缺省自动读取。
- **错误透出**：保留 Content-Type/文本错误，让调用侧（LLM/人）能看到原始返回。
- **防空/防 null**：新增字段时优先 `#[serde(default, deserialize_with = "null_default")]` 对集合容错。
- **发布流程**：`cargo build --release` → 复制二进制到 `C:\coding\envs\mcps\zerotier-mcp.exe`。
- **诊断**：遇到云端解析失败时，添加日志打印响应头与 body 片段；可增加诊断工具返回原文。

## 常见错误点
- Central 返回 `null` 导致 Vec/Map 解析失败 —— 已通过 `null_default` 修复。
- 成功但空/非 JSON 响应导致解码失败 —— `request` 已检查空体与 Content-Type。
- 未配置 `ZEROTIER_CENTRAL_TOKEN` 时调用云端工具会直接提示“未配置 Central API Token”。

## 扩展建议
- 为 MCP 加一个诊断工具返回原始响应摘要（状态码、Content-Type、body 片段）。
- 针对敏感操作（删除网络/成员）添加确认或只读模式开关。
