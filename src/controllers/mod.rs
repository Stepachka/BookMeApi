use axum::Router;
use crate::controllers::post_controller::init_routes_post;
use crate::controllers::user_controller::init_routes_user;
use crate::lib::lib::AppState;

pub mod post_controller;
mod user_controller;

pub fn init_routes_all (config: &AppState) -> Router {
    Router::new()
        .nest("/post", init_routes_post(config))
        .nest("/user", init_routes_user(config))
}