use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token};

use crate::config::Configuration;

#[derive(Debug)]
pub struct SpotifyWrapper(AuthCodeSpotify);

impl SpotifyWrapper {
    pub fn new() -> Self {
        let config = Configuration::read();

        // Please notice that protocol of redirect_uri, make sure it's http (or
        // https). It will fail if you mix them up.
        let oauth = OAuth {
            scopes: scopes!("playlist-modify-private"),
            redirect_uri: format!("http://localhost:{}/callback/spotify", config.app.port),
            ..Default::default()
        };

        let creds = Credentials::new(&config.spotify.client_id, &config.spotify.client_secret);
        Self(AuthCodeSpotify::with_config(
            creds,
            oauth,
            Config {
                token_cached: false,

                ..Default::default()
            },
        ))
    }

    pub fn from_token(token: Token) -> Self {
        Self(AuthCodeSpotify::from_token(token))
    }
}

impl std::ops::Deref for SpotifyWrapper {
    type Target = AuthCodeSpotify;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
