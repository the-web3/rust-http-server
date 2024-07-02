use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::users::{NewUser, User};
use crate::utils::{hash_password, verify_password, generate_token};
use crate::db::Pool;
use crate::models::blog::Blog;

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(pool: web::Data<Pool>, data: web::Json<RegisterRequest>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let hashed_password = hash_password(&data.password).unwrap();
    let new_user = NewUser {
        id: &data.id,
        username: &data.username,
        password: &hashed_password,
    };

    let user_db = diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .get_result::<User>(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(user_db)
}

pub async fn login(pool: web::Data<Pool>, data: web::Json<LoginRequest>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let mut conn = pool.get().unwrap();

    let user_db = users
        .filter(username.eq(&data.username))
        .first::<User>(&mut conn)
        .unwrap();

    if verify_password(&data.password, &user_db.password).unwrap() {
        let token = generate_token(&user_db.id.to_string()).unwrap();
        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
