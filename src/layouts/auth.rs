use gpui::{prelude::*, *};
use std::sync::Arc;

use crate::{components::branding::Button, stores::AppState};

#[derive(IntoElement)]
pub struct AuthLayout {
  app_state: Arc<AppState>,
}

impl RenderOnce for AuthLayout {
  fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
    // page as a whole
    div()
      .size_full()
      .bg(gpui::white())
      .flex()
      .justify_center()
      .items_center()
      .child(
        // authentication widget centered
        div()
          .max_w(px(480.))
          .w_full()
          .flex()
          .flex_col()
          .items_center()
          .rounded_lg()
          .bg(rgb(0x323339))
          .border_1()
          .border_color(rgb(0xadaeb4))
          .p_8()
          .gap_5()
          .child(
            // texts at the top of the widget
            div()
              .flex()
              .flex_col()
              .items_center()
              .gap_1()
              .child(
                div()
                  .text_color(gpui::white())
                  .text_size(px(24.))
                  .font_weight(FontWeight::SEMIBOLD)
                  .child("Welcome back!"),
              )
              .child(
                div()
                  .text_color(rgb(0xdfe0e2))
                  .text_size(px(16.))
                  .child("We're so excited to see you again!"),
              ),
          )
          .child(
            Button::new("login_button")
              .w_full()
              .label("Log In (DEMO TOKEN)")
              .on_click(move |_, _, cx| {
                self.app_state.auth_store.update(cx, |state, _| {
                  state.tokens.push(SharedString::from("ey..."));
                })
              }),
          ),
      )
  }
}

impl AuthLayout {
  pub fn new(app_state: Arc<AppState>) -> Self {
    Self { app_state }
  }
}
