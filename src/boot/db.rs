use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres};
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgPoolOptions;
use color_eyre::Result;

pub static DATABASE_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

async fn init_db_pool(){
    let db = &super::global().database;
    let pool = PgPoolOptions::new()
        .min_connections(db.min)
        .max_connections(db.max)
        .connect(&db.url)
        .await
        .unwrap();
    let _ = DATABASE_POOL.set(pool).is_ok();
    println!("datasource: {}\n       max: {}", db.url, db.max);
}

#[allow(dead_code)]
pub async fn get_pool() -> &'static Pool<Postgres> {
    let pool =  DATABASE_POOL.get();
    let pool = match pool {
        None => {
            init_db_pool().await;
            DATABASE_POOL.get().unwrap()
        }
        Some(pl) => {
            pl
        }
    };

    pool
}

pub async fn get_connect() -> Result<PoolConnection<Postgres>> {
    let pool = get_pool().await;
    Ok(pool.acquire().await?)
}