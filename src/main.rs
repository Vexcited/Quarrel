use gpui::{
    App, Application, Bounds, Context, IntoElement, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, rgba, size,
};

#[derive(IntoElement)]
struct ServerSideBar {}

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

#[derive(IntoElement)]
struct PrivateChannelsBar {}

impl RenderOnce for PrivateChannelsBar {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id("private_channels_side_bar")
            .overflow_y_scroll()
            .w(px(288.))
            .h_full()
            .child(Self::render_tab("Friends"))
            .child(Self::render_tab("Nitro"))
            .child(Self::render_tab("Shop"))
    }
}

impl PrivateChannelsBar {
    pub fn new() -> Self {
        Self {}
    }

    fn render_tab(name: &'static str) -> impl IntoElement {
        div()
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
            .child(name)
    }
}

struct AppLayout {}

impl Render for AppLayout {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0x121214))
            .size_full()
            .flex()
            .child(ServerSideBar::new())
            .child(PrivateChannelsBar::new())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.), px(600.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AppLayout {}),
        )
        .unwrap();

        cx.activate(true);
    });
}
