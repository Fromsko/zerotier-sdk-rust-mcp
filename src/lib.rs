//! ZeroTier SDK for Rust
//!
//! 本 SDK 包含两个子模块：
//! - `client`: 本地 Service API（localhost:9993）
//! - `central`: 云端 Central API（api.zerotier.com）
//!
//! # 快速开始
//!
//! ```rust,no_run
//! use zerotier_sdk::{Client, Central};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 本地节点管理
//!     let local = Client::new();
//!     let status = local.status().await?;
//!
//!     // 云端管理
//!     let cloud = Central::new("your_api_token");
//!     let networks = cloud.networks().list().await?;
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod central;
pub mod mcp;

// 重导出主要类型
pub use client::Client;
pub use central::Central;
pub use mcp::McpServer;
