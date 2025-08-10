// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::{prelude::*, *};
use std::sync::Arc;

use crate::{
  layouts::LayoutSwitcher,
  stores::{AppState, auth::AuthStore},
};

mod api;
mod components;
mod layouts;
mod stores;

fn main() {
  let app = Application::new();

  app.run(move |cx| {
    let bounds = Bounds::centered(None, size(px(900.), px(600.)), cx);
    let opts: WindowOptions = WindowOptions {
      window_bounds: Some(WindowBounds::Windowed(bounds)),
      titlebar: Some(TitlebarOptions {
        title: Some(SharedString::from("Quarrel")),
        ..Default::default()
      }),
      ..Default::default()
    };

    cx.open_window(opts, |_window: &mut Window, cx: &mut App| {
      cx.new(|cx| {
        cx.activate(true);

        let auth_store = cx.new(AuthStore::new);
        let app_state = Arc::new(AppState { auth_store });

        LayoutSwitcher::new(app_state)
      })
    })
    .unwrap();
  });
}
