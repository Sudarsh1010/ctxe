use crate::{
    application::services::token_counter::TokenCounter, domain::common::token::TokenCount,
};

pub struct TokenCountCommand {
    pub input: String,
}

impl TokenCountCommand {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

pub struct TokenCountResult {
    pub text_preview: String,
    pub token_count: TokenCount,
}

pub fn handle_token_count(
    cmd: TokenCountCommand,
    counter: &TokenCounter,
) -> crate::error::Result<TokenCountResult> {
    let token_count = counter.count(&cmd.input);

    let preview = if cmd.input.len() > 50 {
        format!("{}...", &cmd.input[..50])
    } else {
        cmd.input.clone()
    };

    Ok(TokenCountResult {
        text_preview: preview,
        token_count,
    })
}
