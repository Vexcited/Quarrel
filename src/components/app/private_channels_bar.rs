use gpui::{prelude::*, *};
use std::sync::Arc;

use crate::stores::AppState;

#[derive(IntoElement)]
pub struct PrivateChannelsBar {
    app_state: Arc<AppState>,
}

impl RenderOnce for PrivateChannelsBar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id("private_channels_side_bar")
            .overflow_y_scroll()
            .w(px(288.))
            .flex_shrink_0()
            .h_full()
            .bg(rgb(0x2b2d31))
            .p_2()
            .flex()
            .flex_col()
            .gap_1()
            .child(self.render_tab("Friends"))
            .child(self.render_tab("Nitro"))
            .child(self.render_tab("Shop"))
    }
}

impl PrivateChannelsBar {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }
    fn render_tab(&self, name: &'static str) -> impl IntoElement {
        let app_state = self.app_state.clone();

        div()
            .id(name)
            .text_color(rgb(0x94959c))
            .rounded_lg()
            .w_full()
            .px_2()
            .py_1()
            .hover(|s| {
                s.text_color(rgb(0xfbfbfb))
                    .bg(rgba(0x97979f14))
                    .cursor_pointer()
            })
            // .on_click(move |_, _, cx: &mut App| {
            //     app_state.auth_store.update(cx, |state, _| {
            //         state.token = Some(name.into());
            //     });
            // })
            .child(name)
    }
}
