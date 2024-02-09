use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize,)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title:  String,
    pub content:  String,
    pub user_id:  Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at:  Option<NaiveDateTime>,
}