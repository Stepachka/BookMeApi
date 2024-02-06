use axum::Router;
use axum::routing::{get, post};
use crate::lib::lib::AppState;
use crate::service::user_service::UserService;

pub fn init_routes_user(app_state: &AppState) -> Router {
    Router::new()
        .route("/test-user", get("test-user"))
}

async find_all (
    config: &AppState,
) ->