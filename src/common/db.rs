use once_cell::sync::OnceCell;
use color_eyre::Result;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime, PoolConfig, Pool, Object};
use tokio_postgres::NoTls;

pub static DATABASE_POOL: OnceCell<Pool> = OnceCell::new();
const DB_DEFAULT_MAX_CONNECTS: usize = 15;

async fn init_db_pool(){
    let db = &super::global_cfg().database;
    let mut cfg = Config::new();
    cfg.host = db.host.clone();
    cfg.port = db.port.clone();
    cfg.user = db.user.clone();
    cfg.password = db.password.clone();
    cfg.dbname = db.dbname.clone();
    cfg.pool = Some(PoolConfig::new(db.max.unwrap_or(DB_DEFAULT_MAX_CONNECTS)));
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Verified });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    DATABASE_POOL.set(pool).ok();
}

#[allow(dead_code)]
pub async fn get_pool() -> &'static Pool {
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

pub async fn get_connect() -> Result<Object> {
    let pool = get_pool().await;
    let client = pool.get().await?;

    Ok(client)
}