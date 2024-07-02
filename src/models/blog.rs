use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::blogs;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "blogs"]
pub struct Blog {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "blogs"]
pub struct NewBlog<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub author_id: &'a str,
}
