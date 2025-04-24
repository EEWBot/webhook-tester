use std::net::SocketAddr;

use axum::{
    Router,
    extract::{Json, Request},
    http::header::HOST,
    response::{IntoResponse, Response},
    routing::post,
};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

async fn send(Json(body): Json<serde_json::Value>) -> Response {
    Json(crate::req::req(body).await.unwrap()).into_response()
}

async fn get(request: Request) -> Response {
    let host = request
        .headers()
        .get(HOST)
        .map(|v| v.to_str().unwrap())
        .unwrap_or("localhost:3001");

    let req_uri = request.uri().to_string();
    let url = format!("http://{host}{req_uri}");

    let token = request.uri().path().split("/").last().unwrap_or("A");

    Json(serde_json::json!({
        "application_id": "1000000000000000000",
        "avatar": null,
        "channel_id": "1000000000000000001",
        "guild_id": "1000000000000000002",
        "id": "1000000000000000003",
        "name": "WebhookTester",
        "type": 1,
        "token": token,
        "url": url,
    }))
    .into_response()
}

pub async fn run(listen: SocketAddr) {
    let app = Router::new().fallback(post(send).get(get)).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_private_network(true),
    );

    let listener = TcpListener::bind(listen)
        .await
        .expect("Failed to bind address");

    println!("Listening on {}...", listen);

    axum::serve(listener, app)
        .await
        .expect("Failed to serve HTTP contents");
}
