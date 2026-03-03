use crate::domain::common::token::{TokenBudget, TokenCount};

pub trait TokenCounterPort: Send + Sync {
    fn count(&self, text: &str) -> TokenCount;
}

pub struct TokenCounterService<C: TokenCounterPort> {
    counter: C,
}

impl<C: TokenCounterPort> TokenCounterService<C> {
    pub fn new(counter: C) -> Self {
        Self { counter }
    }

    pub fn count(&self, text: &str) -> TokenCount {
        self.counter.count(text)
    }

    pub fn check_budget(
        &self,
        text: &str,
        budget: TokenBudget,
    ) -> crate::Result<TokenCount> {
        let count = self.counter.count(text);

        if count.exceeds(budget) {
            Err(crate::Error::TokenBudget {
                current: count.as_u32() as usize,
                budget: budget.as_u32() as usize,
            })
        } else {
            Ok(count)
        }
    }
}
