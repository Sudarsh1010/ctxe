use rmcp::{ServerHandler, ServiceExt, model::*, transport::stdio};

pub struct CtxeServer;

impl ServerHandler for CtxeServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::default())
            .with_protocol_version(ProtocolVersion::V_2025_06_18)
            .with_instructions(
                "CTXE - Structural Code RAG for intelligent code understanding",
            )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    CtxeServer.serve(stdio()).await?;
    Ok(())
}
