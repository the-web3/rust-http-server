use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::users::User;
use crate::db::Pool;

pub async fn get_user_info(pool: web::Data<Pool>, user_id: web::Path<String>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let mut conn = pool.get().unwrap();

    let user = users
        .filter(id.eq(user_id.into_inner()))
        .first::<User>(&mut conn)
        .unwrap();

    HttpResponse::Ok().json(user)
}
