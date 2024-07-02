use std::env;

pub struct Config {
    pub server_addr: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}
