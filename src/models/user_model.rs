use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(
Queryable,
Selectable,
PartialEq,
Insertable,
AsChangeset,
Identifiable,
Serialize,
Deserialize
)]
#[diesel(table_name = "users")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}