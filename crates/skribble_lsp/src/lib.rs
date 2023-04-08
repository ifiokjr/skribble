use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tower_lsp::LanguageServer;

#[derive(Debug)]
pub struct SkribbleLanguageServer {
  client: Client,
}

impl SkribbleLanguageServer {
  pub fn new(client: Client) -> Self {
    Self { client }
  }
}

#[tower_lsp::async_trait]
impl LanguageServer for SkribbleLanguageServer {
  async fn initialize(&self, _: InitializeParams) -> LspResult<InitializeResult> {
    Ok(InitializeResult::default())
  }

  async fn initialized(&self, _: InitializedParams) {
    self
      .client
      .log_message(MessageType::INFO, "server initialized!")
      .await;
  }

  async fn shutdown(&self) -> LspResult<()> {
    Ok(())
  }
}
