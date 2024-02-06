use axum::Router;
use axum::routing::get;
use crate::lib::lib::AppState;

pub fn init_routes_post(config: &AppState) -> Router {
    Router::new()
        .route("/test-post", get({"test-post"}))
}
