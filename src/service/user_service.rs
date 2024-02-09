use axum::http::StatusCode;
use diesel::{PgConnection, RunQueryDsl, sql_query};
use crate::models::user_model::User;

pub struct UserService {}

impl UserService {
    pub fn get_all(
        conn: &mut PgConnection,
    ) -> Result<Vec<User>, diesel::result::Error> {
        Ok(
            sql_query("select * from users")
                .load::<User>(conn)
                .expect("users fetch error!"),
        )
    }
}