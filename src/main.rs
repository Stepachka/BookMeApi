extern crate diesel;
mod controllers;
mod service;
mod lib;
mod schema;
mod models;

use axum::{Router};
use axum::routing::get;
use axum_swagger_ui::swagger_ui;
use crate::lib::lib::AppState;

#[tokio::main]
async fn main() {

    let doc_url = "swagger/openapi.json";
    let config = AppState {
        pool: AppState::sync_pool()
    };

    let routes = Router::new()
        .route("/swagger", get(|| async { swagger_ui(doc_url) }))
        .route("/", get(test))
        .nest("/api", controllers::init_routes_all(&config));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();

    println!("Server running on port 3000");

    axum::serve(listener, routes).await.unwrap();
}

async fn test() -> &'static str {
    println!("->> Common test");
    return "test"
}
