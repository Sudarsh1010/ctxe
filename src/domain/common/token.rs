use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BudgetError {
    #[error("Budget cannot be zero")]
    ZeroBudget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenCount(u32);

impl TokenCount {
    pub fn new(count: u32) -> Self {
        Self(count) // No validation - allows 0
    }
    pub fn as_u32(self) -> u32 {
        self.0
    }
    pub fn exceeds(self, budget: TokenBudget) -> bool {
        self.0 > budget.0
    }
}

impl Display for TokenCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenBudget(u32);

impl TokenBudget {
    pub fn new(budget: u32) -> Result<Self, BudgetError> {
        if budget == 0 {
            return Err(BudgetError::ZeroBudget);
        }
        Ok(Self(budget))
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}
