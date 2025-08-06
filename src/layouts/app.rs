use gpui::{prelude::*, *};
use std::sync::Arc;

use crate::components::app::{Channel, PrivateChannelsBar, ServerSideBar};
use crate::stores::AppState;

#[derive(IntoElement)]
pub struct AppLayout {
    app_state: Arc<AppState>,
}

impl RenderOnce for AppLayout {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .bg(rgb(0x121214))
            .size_full()
            .flex()
            .child(ServerSideBar::new())
            .child(PrivateChannelsBar::new(self.app_state.clone()))
            .child(Channel::new(self.app_state.clone()))
    }
}

impl AppLayout {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }
}
