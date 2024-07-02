#[macro_use]
extern crate diesel;

mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod schema;
mod utils;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config::Config::new();
    let pool = db::create_pool(&config.database_url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::init)
    })
        .bind(config.server_addr)?
        .run()
        .await
}
