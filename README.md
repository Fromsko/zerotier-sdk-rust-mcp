# ZeroTier SDK for Rust

ZeroTier API 的 Rust SDK，支持 MCP（Model Context Protocol）集成。

## 功能特性

- **本地 Service API** (`client`): 管理本地 ZeroTier 节点（localhost:9993）
- **云端 Central API** (`central`): 管理云端网络（api.zerotier.com）
- **MCP 服务**: 提供 MCP 工具集成

## 快速开始

### 作为库使用

```rust
use zerotier_sdk::{Client, Central};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 本地节点管理
    let local = Client::new();
    let status = local.status().await?;
    println!("节点地址: {}", status.address);

    // 列出已加入的网络
    let networks = local.networks().list().await?;
    for net in networks {
        println!("网络: {} - {}", net.id, net.name);
    }

    // 云端管理
    let cloud = Central::new("your_api_token");
    let networks = cloud.networks().list().await?;
    
    Ok(())
}
```

### 作为 MCP 服务运行

```bash
# 设置 Central API Token（可选）
export ZEROTIER_CENTRAL_TOKEN="your_token"

# 运行 MCP 服务
cargo run --bin zerotier-mcp
```

## MCP 工具列表

### 本地 API 工具

| 工具名 | 描述 |
|--------|------|
| `zt_status` | 获取本地节点状态 |
| `zt_networks` | 列出已加入的网络 |
| `zt_join` | 加入网络 |
| `zt_leave` | 离开网络 |
| `zt_peers` | 列出所有 Peers |

### 云端 API 工具

| 工具名 | 描述 |
|--------|------|
| `zt_central_networks` | 列出云端网络 |
| `zt_central_members` | 列出网络成员 |
| `zt_central_authorize` | 授权成员 |
| `zt_central_deauthorize` | 取消授权 |

## MCP 配置示例

```json
{
  "mcpServers": {
    "zerotier": {
      "command": "zerotier-mcp",
      "env": {
        "ZEROTIER_CENTRAL_TOKEN": "your_token"
      }
    }
  }
}
```

## 模块结构

```
zerotier-sdk/
├── src/
│   ├── lib.rs           # 库入口
│   ├── client/          # 本地 Service API
│   │   ├── mod.rs
│   │   ├── types.rs
│   │   ├── network.rs
│   │   ├── peer.rs
│   │   └── controller.rs
│   ├── central/         # 云端 Central API
│   │   ├── mod.rs
│   │   ├── types.rs
│   │   ├── network.rs
│   │   └── member.rs
│   ├── mcp/             # MCP 服务
│   │   ├── mod.rs
│   │   └── server.rs
│   └── bin/
│       └── zerotier-mcp.rs
```

## License

MIT
