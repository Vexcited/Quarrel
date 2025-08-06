pub mod auth;

use auth::AuthStore;
use gpui::Entity;

pub struct AppState {
    pub auth_store: Entity<AuthStore>,
}
