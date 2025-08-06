use gpui::*;
use std::sync::Arc;

pub mod app;
pub mod auth;

use crate::{
  layouts::{app::AppLayout, auth::AuthLayout},
  stores::AppState,
};

pub struct LayoutSwitcher {
  app_state: Arc<AppState>,
}

impl Render for LayoutSwitcher {
  fn render(
    &mut self,
    _: &mut Window,
    cx: &mut Context<Self>,
  ) -> impl IntoElement {
    let auth_store = self.app_state.auth_store.read(cx);
    let view: AnyElement = if auth_store.has_account() {
      AppLayout::new(self.app_state.clone()).into_any_element()
    } else {
      AuthLayout::new(self.app_state.clone()).into_any_element()
    };

    view
  }
}

impl LayoutSwitcher {
  pub fn new(app_state: Arc<AppState>) -> Self {
    Self { app_state }
  }
}
