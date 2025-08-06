use gpui::{Context, SharedString};

pub struct AuthStore {
  pub tokens: Vec<SharedString>,
}

impl AuthStore {
  pub fn new(_: &mut Context<Self>) -> Self {
    Self { tokens: Vec::new() }
  }

  pub fn has_account(&self) -> bool {
    !self.tokens.is_empty()
  }

  pub fn has_mutiple_accounts(&self) -> bool {
    self.tokens.len() > 1
  }
}
