use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{web, App, HttpServer};
use common::config::AppConfig;
use log::{error, info};
use middleware::error_handler::render_default_error;
use sea_orm::Database;
use services::AppState;
use std::sync::Arc;

mod controller;
mod middleware;

const MIDDLEWARE_LOG_PATTERN: &str = r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i""#;
const LOCAL_ADDRESS: &str = "127.0.0.1";

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    let config = AppConfig::from_env().expect("Load app configuration failed.");
    info!("Load app configuration from /application.yaml");
    let database = Database::connect(&config.database.url)
        .await
        .expect("Connect to database failed.");
    info!("Connect to database from url: {}", config.database.url);

    let app_state = web::Data::new(AppState {
        database: Arc::new(database),
    });

    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(ErrorHandlers::new().default_handler(render_default_error))
            .wrap(Logger::new(MIDDLEWARE_LOG_PATTERN))
            .configure(controller_init)
    })
    .bind((LOCAL_ADDRESS, config.server.port))?;

    app.run().await?;
    Ok(())
}

fn controller_init(cfg: &mut web::ServiceConfig) {
    cfg.configure(controller::template::init);
}

pub fn main() {
    let result = start();
    if let Some(err) = result.err() {
        error!("Start Application failed: {}", err);
    }
}
