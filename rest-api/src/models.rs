use serde::{Serialize, Deserialize};
use super::schema::users;

#[derive(Queryable, Serialize, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub username: String,
}