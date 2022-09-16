use std::{sync::Arc};

use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime, PoolConfig};
use tokio_postgres::{NoTls, Transaction};

#[tokio::main]
async fn main() {
    let mut cfg = Config::new();
    cfg.host = Some("127.0.0.1".to_owned());
    cfg.port = Some(5432);
    cfg.user = Some("root".to_owned());
    cfg.password = Some("111111".to_owned());
    cfg.dbname = Some("ractiviti".to_owned());
    cfg.pool = Some(PoolConfig::new(15));
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let pool = Arc::new(pool);
    
    // 异步测试
    let mut hander_array = vec![];
    for i in 1..5 {
        let pl = pool.clone();
        let join_handle = tokio::spawn( async move {
            let client = pl.get().await.unwrap();
            let stmt = client.prepare_cached("SELECT $1::TEXT").await.unwrap();
            let rows = client.query(&stmt, &[&i.to_string()]).await.unwrap();
            let value: String = rows[0].get(0);
            assert_eq!(value, i.to_string());
            println!("{} - {:?}", value, pl.status())
        });
        hander_array.push(join_handle);
    }

    for j in hander_array {
        j.await.unwrap();
    }

    // Dao 事务测试
    let pl = pool.clone();
    let mut client = pl.get().await.unwrap();
    let tran = client.transaction().await.unwrap();

    let dao_a = DaoA::new(&tran);
    let dao_b = DaoB::new(&tran);

    let _ = dao_a.create_row().await;
    let _ = dao_b.create_row().await;
    
    // tran.commit().await.unwrap();
    tran.rollback().await.unwrap();

}

struct DaoA<'a> {
    tran: &'a Transaction<'a>,
}

struct DaoB<'a> {
    tran: &'a Transaction<'a>,
}

impl<'a> DaoA<'a> {
    pub fn new(tran: &'a Transaction) -> Self {
        Self {
            tran
        }
    }

    pub async fn create_row(&self) -> String {
        let sql = "INSERT INTO test (name) VALUES ($1) RETURNING *";
        let stmt = self.tran.prepare(sql).await.unwrap();
        let rows = self.tran.query(&stmt, &[&"evan"]).await.unwrap();
        let value: String = rows[0].get(0);
        println!("{}", value);

        value
    }
}

impl<'a> DaoB<'a> {
    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            tran
        }
    }

    pub async fn create_row(&self) -> String {
        let sql = "INSERT INTO test (name) VALUES ($1) RETURNING *";
        let stmt = self.tran.prepare(sql).await.unwrap();
        let rows = self.tran.query(&stmt, &[&"athena"]).await.unwrap();
        let value: String = rows[0].get(0);
        println!("{}", value);

        value
    }
}