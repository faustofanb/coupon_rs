pub mod template_dao;

use crate::config::DatabaseConfig;
use crate::storage::entity::template_entity::TemplateDO;
use actix_web::cookie::time::ext::NumericalStdDuration;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, MySqlPool};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct Table<'c, T>
where
    T: FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'c MySqlRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

// pub struct JoinTable<'c, T1, T2>
// where T1: FromRow<'c, MySqlRow>, T2: FromRow<'c, MySqlRow> {
//     pub pool: MySqlPool,
//     _from_row: (
//         fn(&'c MySqlRow) -> Result<T1, sqlx::Error>,
//         fn(&'c MySqlRow) -> Result<T2, sqlx::Error>
//     ),
//     _marker_t1: PhantomData<&'c T1>,
//     _marker_t2: PhantomData<&'c T2>,
// }
//
// impl<'c, T1, T2> JoinTable<'c, T1, T2>
// where T1: FromRow<'c, MySqlRow>, T2: FromRow<'c, MySqlRow> {
//     fn new(pool: MySqlPool) -> Self {
//         JoinTable {
//             pool,
//             _from_row: (T1::from_row, T2::from_row),
//             _marker_t1: PhantomData,
//             _marker_t2: PhantomData,
//         }
//     }
// }

pub struct Database<'c> {
    pub template: Arc<Table<'c, TemplateDO>>,
}

impl<'c> Database<'c> {
    pub async fn new(database_config: &DatabaseConfig) -> Database<'c> {
        let pool = MySqlPoolOptions::new()
            .max_connections(database_config.max_connections)
            .min_connections(database_config.min_connections)
            .acquire_timeout(database_config.connect_timeout_seconds.std_seconds())
            .idle_timeout(database_config.idle_timeout_seconds.std_seconds())
            .connect(&database_config.url)
            .await
            .expect("Failed to connect to database");

        Database {
            template: Arc::from(Table::new(Arc::new(pool).clone())),
        }
    }
}
