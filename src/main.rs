mod controllers;
mod service;
mod lib;
mod schema;

use axum::{Router};
use axum::routing::get;
use crate::lib::lib::AppState;
use axum::extract::Extension;

#[tokio::main]
async fn main() {

    let config = AppState {
        pool: AppState::new()
    };
    let routes = Router::new()
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
