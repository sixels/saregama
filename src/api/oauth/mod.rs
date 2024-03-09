pub mod extractor;
mod spotify;

use axum::{routing::get, Router};

use crate::AppState;

pub struct OAuthService {}

impl OAuthService {
    pub fn router(state: AppState) -> Router {
        let mut router = Router::new()
            .route("/spotify", get(spotify::spotify_callback))
            .route("/ytmusic", get(ytmusic_callback));

        if cfg!(debug_assertions) {
            router = router.route("/test-spotify", get(spotify::test_callback));
        }

        router.with_state(state)
    }
}

async fn ytmusic_callback() {}
