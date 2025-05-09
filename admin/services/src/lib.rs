use std::sync::Arc;
use sea_orm::DatabaseConnection;

pub mod template;
pub mod dto;
pub mod auth;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: Arc<DatabaseConnection>,
}

