use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::{Cookie, Expiration, SameSite, SignedCookieJar};
use rspotify::clients::{BaseClient, OAuthClient};
use serde::Deserialize;
use time::{ext::NumericalDuration, OffsetDateTime};
use tracing::info;

use crate::extensions::spotify::SpotifyWrapper;

use super::extractor::OauthExtractor;

#[derive(Deserialize)]
pub struct SpotifyCallbackQuery {
    code: String,
}

pub async fn spotify_callback(
    jar: SignedCookieJar,
    query: Query<SpotifyCallbackQuery>,
) -> Response {
    let code = query.code.clone();
    info!(code = ?code, "handling spotify oauth callback");

    let spotify = SpotifyWrapper::new();

    match spotify.request_token(&code).await {
        Ok(_) => {
            info!("successfully authenticated with spotify");

            let token = spotify.get_token().lock().await.unwrap().clone().unwrap();

            let serialized_token = serde_json::to_string(&token).unwrap();

            (
                jar.add(
                    Cookie::build(("spotify_token", serialized_token))
                        .secure(true)
                        .domain("localhost")
                        .path("/")
                        .expires(Expiration::DateTime(
                            OffsetDateTime::now_utc().saturating_add(7.days()),
                        ))
                        .http_only(true)
                        .same_site(SameSite::Lax),
                ),
                Redirect::to("/"),
            )
                .into_response()
        }
        Err(e) => {
            info!(error = ?e, "failed to authenticate with spotify");
            "failed to authenticate with spotify".into_response()
        }
    }
}

#[cfg(debug_assertions)]
pub async fn test_callback(oauth: OauthExtractor) -> Response {
    if oauth.spotify().is_none() {
        let spotify = SpotifyWrapper::new();
        let auth_url = spotify.get_authorize_url(true).unwrap();

        info!(auth_url = %auth_url, "redirecting to spotify for authentication");

        Redirect::temporary(&auth_url).into_response()
    } else {
        Redirect::permanent("/").into_response()
    }
}
