use std::marker::PhantomData;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::{cookie::Key, SignedCookieJar};
use rspotify::Token;

use crate::extensions::spotify::SpotifyWrapper;

#[derive(Debug)]
pub struct OauthState {
    spotify: Option<SpotifyWrapper>,
}

#[derive(Debug)]
pub struct OauthExtractor<K = Key> {
    auth_state: OauthState,
    _mark: PhantomData<K>,
}

impl<K> OauthExtractor<K> {
    pub fn spotify(&self) -> Option<&SpotifyWrapper> {
        self.auth_state.spotify.as_ref()
    }
}

#[async_trait]
impl<S, K> FromRequestParts<S> for OauthExtractor<K>
where
    S: Send + Sync,
    K: FromRef<S> + Into<Key>,
{
    type Rejection = <SignedCookieJar<K> as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = SignedCookieJar::<K>::from_request_parts(parts, state).await?;

        let spotify_token = jar
            .get("spotify_token")
            .and_then(|token| serde_json::from_str::<Token>(token.value()).ok());

        let auth_state = OauthState {
            spotify: match spotify_token {
                Some(token) => Some(SpotifyWrapper::from_token(token)),
                None => None,
            },
        };

        Ok(Self {
            auth_state,
            _mark: Default::default(),
        })
    }
}
