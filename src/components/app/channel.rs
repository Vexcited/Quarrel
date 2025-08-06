use gpui::{prelude::*, *};
use std::sync::Arc;

use crate::stores::AppState;

#[derive(IntoElement)]
pub struct Channel {
    app_state: Arc<AppState>,
}

impl Channel {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }
}

impl RenderOnce for Channel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let auth = self.app_state.auth_store.read(cx);

        div()
            .id("main_channel")
            .h_full()
            .flex_1()
            .bg(rgb(0x313338))
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(48.))
                    .flex_shrink_0()
                    .bg(rgb(0x2b2d31))
                    .border_b_1()
                    .border_color(rgb(0x1e1f22))
                    .flex()
                    .items_center()
                    .px_4()
                    .child(
                        div()
                            .text_color(rgb(0xfbfbfb))
                            .text_lg()
                            .font_weight(gpui::FontWeight::SEMIBOLD), // .when_some(auth.token.clone(), |this, token| {
                                                                      //     this.child(format!("you're authenticated with {token}"))
                                                                      // }),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child("Channel content goes here...")
                    .text_color(rgb(0xdbdee1)),
            )
    }
}
