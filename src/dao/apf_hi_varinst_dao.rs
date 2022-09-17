use chrono::NaiveDateTime;
use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;
use crate::error::{AppError, ErrorCode};
use crate::model::{ApfHiVarinst, ApfRuVariable, NewApfHiVarinst};

use super::{BaseDao, Dao};

pub struct ApfHiVarinstDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfHiVarinstDao<'a> {

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfHiVarinstDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }

    pub async fn create_by_variable(&self, variable: &ApfRuVariable, create_time: NaiveDateTime) -> Result<ApfHiVarinst> {
        let hi_new_var = NewApfHiVarinst {
            id: variable.id.clone(),
            var_type: variable.var_type.clone(),
            name: variable.name.to_owned(),
            value: variable.value.to_owned(),
            proc_inst_id: variable.proc_inst_id.clone(),
            execution_id: variable.execution_id.clone(),
            task_id: variable.task_id.clone(),
            create_time,
            last_updated_time: create_time
        };

        let rst = self.create(&hi_new_var).await?;

        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfHiVarinst) -> Result<ApfHiVarinst> {
        let sql = r#"
            insert into apf_hi_varinst (
                rev, id, var_type, name, value, proc_inst_id, execution_id, task_id, create_time, last_updated_time
            ) values (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
            returning *
        "#;

        let rev:i32 = 1;
        let stmt = self.tran().prepare(sql).await?;
        let row = self
            .tran()
            .query_one(
                &stmt, 
                &[
                    &rev,
                    &obj.id,
                    &obj.var_type,
                    &obj.name,
                    &obj.value,
                    &obj.proc_inst_id,
                    &obj.execution_id,
                    &obj.task_id,
                    &obj.create_time,
                    &obj.last_updated_time,
                ]
            )
            .await?;
        let rst = ApfHiVarinst::from_row(row)?;

        Ok(rst)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<ApfHiVarinst> {
        let sql = r#"
            select id, rev, var_type, name, value,
                proc_inst_id, execution_id, task_id, create_time, last_updated_time 
            from apf_hi_varinst
            where id = $1
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let rows = self.tran().query(&stmt, &[&id]).await?;
        if rows.len() == 0 {
            Err(
                AppError::new(
                    ErrorCode::NotFound, 
                    Some(&format!("apf_hi_varinst(id:{}) is not exist", id)), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        let rst = ApfHiVarinst::from_row_ref(&rows[0])?;

        Ok(rst)
    }

    pub async fn create_or_update_by_variable(&self, variable: &ApfRuVariable, update_time: NaiveDateTime) -> Result<ApfHiVarinst> {
        let mut rst_hi_var = self.get_by_id(&variable.id).await;
        match &mut rst_hi_var {
            Ok(_) => {
                // do update
                self.update_by_variable(variable, update_time).await?;
            },
            Err(error) => {
                let err = error.downcast_ref::<AppError>()
                    .ok_or(AppError::internal_error(&format!("{} : {} , {}", file!(), line!(), error.to_string())))?; 

                if ErrorCode::NotFound == err.code {
                    // do create
                    self.create_by_variable(variable, update_time).await?;
                }
            }
        };

        let hi_var = self.get_by_id(&variable.id).await?;

        Ok(hi_var)
    }

    pub async fn update_by_variable(&self, variable: &ApfRuVariable, update_time: NaiveDateTime) -> Result<u64> {
        let mut hi_var = self.get_by_id(&variable.id).await?;
        hi_var.var_type = variable.var_type.clone();
        hi_var.value = variable.value.clone();
        hi_var.name = variable.name.clone();
        hi_var.last_updated_time = update_time;

        let r = self.update(&hi_var).await?;

        Ok(r)
    }

    pub async fn update(&self, obj: &ApfHiVarinst) -> Result<u64> {
        let sql = r#"
            update apf_hi_varinst
            set rev = rev + 1,
                var_type = $1,
                name = $2,
                value = $3,
                execution_id = $4,
                task_id = $5,
                last_updated_time = $6
            where id = $7
                and rev = $8
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self
            .tran()
            .execute(
                &stmt, 
                &[
                    &obj.var_type,
                    &obj.name,
                    &obj.value,
                    &obj.execution_id,
                    &obj.task_id,
                    &obj.last_updated_time,
                    &obj.id,
                    &obj.rev,
                ]
            )
            .await?;

        if r != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError, 
                    Some(&format!("apf_hi_varinst({}) is not updated correctly, affects ({}) != 1", obj.id, r)), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(r)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::boot::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::dao::apf_ru_variable_dao::tests::create_test_apf_ru_var;
    use crate::get_now;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::WrappedValueType;

    use super::*;

    #[tokio::test]
    async fn test_create_and_update() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let proc_inst = create_test_procinst(&procdef, &tran).await;
        let task = create_test_task(&proc_inst, &tran).await;

        let mut ru_var = create_test_apf_ru_var(&task, &tran).await;
        assert_eq!(ru_var.proc_inst_id, task.proc_inst_id);

        let hi_var_dao = ApfHiVarinstDao::new(&tran);
        let hi_var = hi_var_dao.create_by_variable(&ru_var, get_now()).await.unwrap();
        assert_eq!(hi_var.id, ru_var.id);

        ru_var.var_type = WrappedValueType::STRING;
        let update_var = "test_update_hi_var";
        ru_var.value = update_var.to_owned();
        hi_var_dao.update_by_variable(&ru_var, get_now()).await.unwrap();
        let hi_var2 = hi_var_dao.get_by_id(&ru_var.id).await.unwrap();
        assert_eq!(hi_var2.id, ru_var.id);
        assert_eq!(hi_var2.value, update_var.to_owned());

        tran.rollback().await.unwrap();
    }

}
