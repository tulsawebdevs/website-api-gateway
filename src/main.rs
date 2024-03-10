mod config;

use axum::{
    body::{Body, Bytes},
    extract::{Path, Request, State},
    response::{IntoResponse, Response},
    routing::any,
    Json, Router,
};
use axum_macros::debug_handler;
use dotenv::dotenv;
use http::{header, StatusCode};
use reqwest::Method;
use tokio::net::TcpListener;

use crate::config::GatewayConfig;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = config::load_config("gateway_config.toml");

    // TODO add middleware to match service name and hit auth middleware if not bypassed for service
    let app = Router::new()
        .route("/:service/*destination", any(handle_inbound_req))
        .with_state(config);

    let host = std::env::var("GATEWAY_HOST").unwrap();
    let port = std::env::var("GATEWAY_PORT").unwrap();
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn handle_inbound_req(
    State(config): State<GatewayConfig>,
    Path((service_name, dest)): Path<(String, String)>,
    inbound_req: Request,
) -> Response {
    match config.services.get(&service_name) {
        Some(service) => {
            let client = reqwest::Client::new();
            let method = Method::from_bytes(inbound_req.method().as_str().as_bytes())
                .expect("Unable to parse request method");

            let req = client.request(
                method,
                format!("{}:{}/{}", service.host, service.port, dest),
            );

            //TODO get responses right - try to pass the reqwest response straight through
            // TODO figure out best way to return a 404 if service name not found
            // ? Might be better to build a middleware that extracts service name and mounts service on the router and 404s otherwise rather than do it in the handler. That way by the time we get to a handler the service is already in the route state. I think this is the way.
            let res = req.send().await.unwrap();
            let res_headers = res.headers();
            let content_type = res_headers.get("Content-Type").unwrap();
            res.bytes().await.unwrap().into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
