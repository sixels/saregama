use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use saregama::{api::oauth, config::Configuration, AppState};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let state = AppState::new();

    // Router::new().route_service(path, service)

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind(format!("0.0.0.0:{}", Configuration::read().app.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
