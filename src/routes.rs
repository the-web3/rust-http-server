use actix_web::web;
use crate::handlers::{auth, users, blogs};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register")
            .route(web::post().to(auth::register))
    );
    cfg.service(
        web::resource("/login")
            .route(web::post().to(auth::login))
    );
    cfg.service(
        web::resource("/user/{id}")
            .route(web::get().to(users::get_user_info))
    );
    cfg.service(
        web::resource("/blog")
            .route(web::post().to(blogs::create_blog))
    );
    cfg.service(
        web::resource("/blog/{id}")
            .route(web::get().to(blogs::get_blog))
            .route(web::put().to(blogs::update_blog))
    );
}
