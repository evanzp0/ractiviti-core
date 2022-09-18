use std::any::{Any, TypeId};

use color_eyre::Result;
use rstring_builder::StringBuilder;
use tokio_postgres::Transaction;

use crate::error::{AppError, ErrorCode};

use super::{BaseDao, Dao};

pub struct CrieriaDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao<'a> for CrieriaDao<'a> {
    fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}


impl<'a> CrieriaDao<'a> {
    pub fn bind_params<'a, T>(&self, mut query: QueryAsType<'a, T>, params: &Vec<Box<dyn Any>>) -> Result<QueryAsType<'a, T>> {
        for param in params {
            let param_type_id = (**param).type_id();

            if TypeId::of::<Option<String>>() == param_type_id {
                let p = param.downcast_ref::<Option<String>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?.clone();
                query = query.bind(p);
            } else if TypeId::of::<Option<i32>>() == param_type_id {
                let p = *param.downcast_ref::<Option<i32>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<i64>>() == param_type_id {
                let p = *param.downcast_ref::<Option<i64>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<f32>>() == param_type_id {
                let p = *param.downcast_ref::<Option<f32>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<f64>>() == param_type_id {
                let p = *param.downcast_ref::<Option<f64>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<i8>>() == param_type_id {
                let p = *param.downcast_ref::<Option<i8>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<i16>>() == param_type_id {
                let p = *param.downcast_ref::<Option<i16>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<bool>>() == param_type_id {
                let p = *param.downcast_ref::<Option<bool>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else {
                query = Self::bind_prarms_raw(query, param)?;
            }
        }

        Ok(query)
    }

    pub fn bind_params_scalar<'a, T>(&self, mut query: QueryScalarAsType<'a, T>, params: &Vec<Box<dyn Any>>) -> Result<QueryScalarAsType<'a, T>> {
        for param in params {
            let param_type_id = (**param).type_id();

            if TypeId::of::<Option<String>>() == param_type_id {
                let p = param.downcast_ref::<Option<String>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p.clone());
            } else if TypeId::of::<Option<i32>>() == param_type_id {
                let p = *param.downcast_ref::<Option<i32>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<i64>>() == param_type_id {
                let p = *param.downcast_ref::<Option<i64>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<f32>>() == param_type_id {
                let p = *param.downcast_ref::<Option<f32>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<f64>>() == param_type_id {
                let p = *param.downcast_ref::<Option<f64>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<i8>>() == param_type_id {
                let p = *param.downcast_ref::<Option<i8>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<i16>>() == param_type_id {
                let p = *param.downcast_ref::<Option<i16>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else if TypeId::of::<Option<bool>>() == param_type_id {
                let p = *param.downcast_ref::<Option<bool>>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                query = query.bind(p);
            } else {
                query = Self::bind_prarms_raw_scalar(query, param)?;
            }
        }

        Ok(query)
    }

    fn bind_prarms_raw<'a, T>(&self, mut query: QueryAsType<'a, T>, param: &Box<dyn Any>) -> Result<QueryAsType<'a, T>> {
        let param_type_id = (**param).type_id();
        if TypeId::of::<String>() == param_type_id {
            let p = param.downcast_ref::<String>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p.clone());
        } else if TypeId::of::<i32>() == param_type_id {
            let p = *param.downcast_ref::<i32>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<i64>() == param_type_id {
            let p = *param.downcast_ref::<i64>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<f32>() == param_type_id {
            let p = *param.downcast_ref::<f32>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<f64>() == param_type_id {
            let p = *param.downcast_ref::<f64>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<i8>() == param_type_id {
            let p = *param.downcast_ref::<i8>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<i16>() == param_type_id {
            let p = *param.downcast_ref::<i16>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<bool>() == param_type_id {
            let p = *param.downcast_ref::<bool>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else {
            Err(
                AppError::new(
                    ErrorCode::NotSupportError, Some("sql bind param not support"), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(query)
    }

    fn bind_prarms_raw_scalar<'a, T>(&self, mut query: QueryScalarAsType<'a, T>, param: &Box<dyn Any>) -> Result<QueryScalarAsType<'a, T>> {
        let param_type_id = (**param).type_id();
        if TypeId::of::<String>() == param_type_id {
            let p = param.downcast_ref::<String>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p.clone());
        } else if TypeId::of::<i32>() == param_type_id {
            let p = *param.downcast_ref::<i32>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<i64>() == param_type_id {
            let p = *param.downcast_ref::<i64>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<f32>() == param_type_id {
            let p = *param.downcast_ref::<f32>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<f64>() == param_type_id {
            let p = *param.downcast_ref::<f64>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<i8>() == param_type_id {
            let p = *param.downcast_ref::<i8>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<i16>() == param_type_id {
            let p = *param.downcast_ref::<i16>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else if TypeId::of::<bool>() == param_type_id {
            let p = *param.downcast_ref::<bool>().ok_or(AppError::unexpected_error(concat!(file!(), ":", line!())))?;
            query = query.bind(p);
        } else {
            Err(
                AppError::new(
                    ErrorCode::NotSupportError, 
                    Some("sql bind param not support"), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(query)
    }

    pub fn split_params(&self, param: &str, spliter_char: char) -> String {
        let param_arr = param.split(spliter_char);
        let mut param_in_str = StringBuilder::new();

        for p in param_arr {
            param_in_str.append("'").append(p.trim()).append("'").append(",");
        }
        param_in_str.delete_at(param_in_str.len() - 1);
        param_in_str.string()
    }

    pub async fn find_by_crieria<T>(&self, sql: &str, params: &Vec<Box<dyn Any>>) -> Result<Vec<T>> {
        let query = self.tran().prepare(sql).await?;
        query = self.bind_params(query, params)?;

        let rst = query.fetch_all(&mut *tran).await?;

        Ok(rst)
    }

    // params: &[&dyn ToSql + Sync]

    pub async fn fetch_one_by_crieria<'a, T>(&self, sql: &str, params: &Vec<Box<dyn Any>>) -> Result<T>
            where T: for<'r> FromRow<'r, PgRow> + Send + Unpin {

        let mut query = sqlx::query_as::<_, T>(sql);
        query = CrieriaDao::bind_params(query, params)?;
        let rst = query.fetch_one(&mut *tran).await?;

        Ok(rst)
    }

    pub async fn fetch_scalar_by_crieria(&self, sql: &str, params: &Vec<Box<dyn Any>>) -> Result<i64> {
        let mut query = sqlx::query_scalar::<_, i64>(sql);
        query = CrieriaDao::bind_params_scalar(query, params)?;
        let rst = query.fetch_one(&mut *tran).await?;

        Ok(rst)
    }

}