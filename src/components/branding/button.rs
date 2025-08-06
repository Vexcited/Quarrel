//! Heavily inspired on https://github.com/longbridge/gpui-component/blob/main/crates/ui/src/button/button.rs

use gpui::{prelude::*, *};

type OnClickFn = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Button {
  base: Stateful<Div>,
  label: Option<SharedString>,
  on_click: Option<OnClickFn>,
  loading: bool,
  disabled: bool,
  stop_propagation: bool,
  w_full: bool,
}

impl Button {
  pub fn new(id: impl Into<ElementId>) -> Self {
    Self {
      base: div().id(id.into()).flex_shrink_0(),
      label: None,
      on_click: None,
      loading: false,
      disabled: false,
      stop_propagation: true,
      w_full: false,
    }
  }

  /// Set label to the Button, if no label is set, the button will be in Icon Button mode.
  pub fn label(mut self, label: impl Into<SharedString>) -> Self {
    self.label = Some(label.into());
    self
  }

  pub fn on_click(
    mut self,
    handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
  ) -> Self {
    self.on_click = Some(Box::new(handler));
    self
  }

  /// Whether the button should extends to its fullest.
  pub fn w_full(mut self) -> Self {
    self.w_full = true;
    self
  }
}

impl RenderOnce for Button {
  fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
    self
      .base
      .cursor_pointer()
      .flex()
      .items_center()
      .justify_center()
      .bg(rgb(0x5865f2))
      .rounded_lg()
      .border_1()
      .border_color(rgba(0xffffff14))
      .px_4()
      .py_3()
      .when(self.w_full, |this| this.w_full())
      .when_some(
        self.on_click.filter(|_| !self.disabled && !self.loading),
        |this, on_click| {
          let stop_propagation = self.stop_propagation;
          this
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
              window.prevent_default();
              if stop_propagation {
                cx.stop_propagation();
              }
            })
            .on_click(move |event, window, cx| {
              (on_click)(event, window, cx);
            })
        },
      )
      .when_some(self.label, |this, label| {
        this.child(
          div()
            .text_color(gpui::white())
            .text_size(px(16.))
            .font_weight(FontWeight::MEDIUM)
            .flex_none()
            .line_height(relative(1.))
            .child(label),
        )
      })
  }
}
