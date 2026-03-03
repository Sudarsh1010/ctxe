//! Unified error handling

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Token budget exceeded: {current} > {budget}")]
    TokenBudget { current: usize, budget: usize },

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Embedding error: {0}")]
    Embedding(String),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Config error: {0}")]
    Config(String),
}

impl From<crate::domain::common::token::BudgetError> for Error {
    fn from(err: crate::domain::common::token::BudgetError) -> Self {
        Error::Config(err.to_string())
    }
}
