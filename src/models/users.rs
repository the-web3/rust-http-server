use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
