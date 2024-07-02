use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::models::blog::{NewBlog, Blog};
use crate::db::Pool;

#[derive(Serialize, Deserialize)]
pub struct CreateBlogRequest {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBlogRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

pub async fn create_blog(pool: web::Data<Pool>, data: web::Json<CreateBlogRequest>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let new_blog = NewBlog {
        id: &data.id,
        title: &data.title,
        content: &data.content,
        author_id: &data.author_id,
    };

    let blog = diesel::insert_into(crate::schema::blogs::table)
        .values(&new_blog)
        .get_result::<Blog>(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(blog)
}

pub async fn update_blog(pool: web::Data<Pool>, blog_id: String, data: web::Json<UpdateBlogRequest>) -> impl Responder {
    use crate::schema::blogs::dsl::*;
    let mut conn = pool.get().unwrap();
    let target = blogs.filter(id.eq(blog_id));

    let updated_blog = diesel::update(target)
        .set((
            data.title.as_ref().map(|t| title.eq(t)),
            data.content.as_ref().map(|c| content.eq(c)),
        ))
        .get_result::<Blog>(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(updated_blog)
}

pub async fn get_blog(pool: web::Data<Pool>, blog_id: web::Path<String>) -> impl Responder {
    use crate::schema::blogs::dsl::*;
    let mut conn = pool.get().unwrap();

    let blog = blogs
        .filter(id.eq(blog_id.into_inner()))
        .first::<Blog>(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(blog)
}
