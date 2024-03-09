pub mod api;
pub mod config;
pub mod extensions;
pub mod ytmusic;

use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use config::Configuration;

#[derive(Debug, Clone)]
pub struct AppState {
    cookie_sign_key: Key,
}

impl AppState {
    pub fn new() -> Self {
        let cookie_secret = &Configuration::app().cookie_secret;

        Self {
            cookie_sign_key: Key::from(cookie_secret.as_bytes()),
        }
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_sign_key.clone()
    }
}
