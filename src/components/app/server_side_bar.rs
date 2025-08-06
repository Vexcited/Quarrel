use gpui::{prelude::*, *};

#[derive(IntoElement)]
pub struct ServerSideBar {}

impl ServerSideBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for ServerSideBar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id("server_side_bar")
            .overflow_y_scroll()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .p_2()
            .gap_2()
            .children((0..20).map(|i| {
                div()
                    .flex_shrink_0()
                    .size_10()
                    .bg(gpui::white())
                    .rounded_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(format!("S{i}"))
            }))
    }
}
