use std::sync::{Arc, Mutex};
use actix_web::{middleware, web, App, HttpServer};
use log::info;
use crate::config::AppConfig;
use crate::storage::dao::Database;

mod controller;
mod config;
mod util;
mod transfer;
mod storage;
mod common;

const MIDDLEWARE_LOG_PATTERN: &str = r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i""#;
const LOCAL_ADDRESS: &str = "127.0.0.1";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = AppConfig::from_env().expect("Load app configuration failed.");
    info!("Load app configuration from config/application.yaml");
    let database = Database::new(&config.database).await;
    info!("Connect to database from url: {}", config.database.url);

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        database: Arc::new(database),
    });
    
    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::new(MIDDLEWARE_LOG_PATTERN))
            .configure(controller_init)
    }).bind((LOCAL_ADDRESS, config.server_port))?;


    app.run().await
}

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub database: Arc<Database<'a>>
}

fn controller_init(cfg: &mut web::ServiceConfig) {
    cfg.configure(controller::template_controller::init);
}