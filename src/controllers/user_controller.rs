use axum::{Json, Router};
use axum::routing::{get, post};
use crate::lib::lib::AppState;
use crate::schema::users::dsl::*;
use serde_json::{json, Value};
use crate::service::user_service::UserService;

pub fn init_routes_user(app_state: &AppState) -> Router {
    Router::new()
        .route("/test-user", get("test-user"))
        .route("/", get(get_users))
}
async fn get_users(
    app_state: AppState
) -> Value {
    println!("->> Handle route - get_users");
    let conn = app_state.pool;
    let users = UserService::get_all(&mut conn.get().unwrap()).unwrap();
    json!(users)

}