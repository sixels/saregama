use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use tokio::net::TcpListener;

use crate::{config::Configuration, AppState};

pub mod oauth;

pub trait ApiService {
    fn base(&self) -> &'static str;
    fn routes(&self) -> axum::Router;
}

pub struct ApiServer {
    services: Vec<Box<dyn ApiService>>,
}

impl ApiServer {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn add_service(&mut self, service: Box<dyn ApiService>) {
        self.services.push(service);
    }

    pub async fn run(self) {
        let config = Configuration::app();

        let state = AppState::new();

        let mut router = Router::new().route("/", get(root));

        for service in self.services {
            router = router.nest(service.base(), service.routes())
        }

        router = router.layer(Extension(state));

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], config.port));
        let server = TcpListener::bind(&addr).await.unwrap();

        axum::serve(server, router).await.unwrap();
    }
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    tracing::event!(
        tracing::Level::INFO,
        ip.addr = "Unknown",
        ip.blocklisted = false,
        "request received",
    );

    "Hello"
}
