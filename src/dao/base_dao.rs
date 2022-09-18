use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Transaction, types::ToSql};

use crate::{common::StringBuilder};

pub trait Dao {
    fn tran(&self) -> &Transaction;
}

pub struct BaseDao<'a> {
    tran: &'a Transaction<'a>,
}

impl Dao for BaseDao<'_> {
    fn tran(&self) -> &Transaction {
        self.tran
    }
}

impl<'a> BaseDao<'a> {
    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            tran
        }
    }

    pub async fn fetcth_all<T>(&self, sql: &str, params: &Vec<&(dyn ToSql + Sync)>) -> Result<Vec<T>> 
    where 
        T : FromTokioPostgresRow  //&[&dyn ToSql + Sync]
    {
        let stmt = self.tran().prepare(sql).await?;
        let params = params.into_iter().as_slice();
        let rows = self.tran().query(&stmt, params).await?;
        let rst = rows
            .iter()
            .map(|row| T::from_row_ref(row).expect("unexpected_error"))
            .collect::<Vec<T>>();

        Ok(rst)
    }

    pub async fn fetch_one<T>(&self, sql: &str, params: &Vec<&(dyn ToSql + Sync)>) -> Result<T>
    where 
        T: FromTokioPostgresRow 
    {
        let stmt = self.tran().prepare(sql).await?;
        let params = params.into_iter().as_slice();
        let row = self.tran().query_one(&stmt, params).await?;
        let rst = T::from_row(row)?;

        Ok(rst)
    }

    pub async fn fetch_i64(&self, sql: &str, params: &Vec<&(dyn ToSql + Sync)>) -> Result<i64> {
        let stmt = self.tran().prepare(sql).await?;
        let params = params.into_iter().as_slice();
        let row = self.tran().query_one(&stmt, params).await?;

        let rst = row.get(0);

        Ok(rst)
    }

    pub fn split_params(param: &str, spliter_char: char) -> String {
        let param_arr = param.split(spliter_char);
        let mut param_in_str = StringBuilder::new();

        for p in param_arr {
            param_in_str.append("'").append(p.trim()).append("'").append(",");
        }
        param_in_str.delete_at(param_in_str.len() - 1);
        param_in_str.string()
    }
}
