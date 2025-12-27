use zerotier_sdk_rust_mcp::McpServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取 Central API Token（可选）
    let central_token = std::env::var("ZEROTIER_CENTRAL_TOKEN").ok();

    let mut server = McpServer::new();
    
    if let Some(token) = central_token {
        server = server.with_central_token(token);
    }

    server.serve_stdio().await?;
    Ok(())
}
