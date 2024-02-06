use crate::schema::users;

#[derive(
Clone,
Serialize,
Deserialize,
Debug,
PartialEq,
Queryable,
Insertable,
AsChangeset,
Identifiable,
QueryableByName,
MetaFields,
)]
#[primary_key(uuid)]
#[table_name = "users"]
pub struct UsersModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at:Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}