use chrono::NaiveDateTime;
use color_eyre::Result;
use sqlx::{Error, Postgres, Transaction};
use crate::error::{AppError, ErrorCode};
use crate::model::{ApfHiVarinst, ApfRuVariable, NewApfHiVarinst};

pub struct ApfHiVarinstDao {

}

impl ApfHiVarinstDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_by_variable(&self, variable: &ApfRuVariable, create_time: NaiveDateTime, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiVarinst> {
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

        let rst = self.create(&hi_new_var, tran).await?;

        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfHiVarinst, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiVarinst> {
        let sql = r#"
            insert into apf_hi_varinst (
                rev, id, var_type, name, value, proc_inst_id, execution_id, task_id, create_time, last_updated_time
            ) values (
                1, $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            returning *
        "#;

        let rst = sqlx::query_as::<_, ApfHiVarinst>(sql)
            .bind(&obj.id)
            .bind(&obj.var_type)
            .bind(&obj.name)
            .bind(&obj.value)
            .bind(&obj.proc_inst_id)
            .bind(&obj.execution_id)
            .bind(&obj.task_id)
            .bind(&obj.create_time)
            .bind(&obj.last_updated_time)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn get_by_id(&self, id: &str, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiVarinst> {
        let sql = r#"
            select id, rev, var_type, name, value,
                proc_inst_id, execution_id, task_id, create_time, last_updated_time 
            from apf_hi_varinst
            where id = $1
        "#;

        let rst = sqlx::query_as::<_, ApfHiVarinst>(sql)
            .bind(id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn create_or_update_by_variable(&self, variable: &ApfRuVariable, update_time: NaiveDateTime, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiVarinst> {
        let mut rst_hi_var = self.get_by_id(&variable.id, tran).await;
        match &mut rst_hi_var {
            Ok(_) => {
                // do update
                self.update_by_variable(variable, update_time, tran).await?;
            },
            Err(error) => {
                // ptln!(error.is::<Error>());
                let err = error.downcast_ref::<Error>().ok_or(
                    AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                if let Error::RowNotFound = err {
                    // do create
                    self.create_by_variable(variable, update_time, tran).await?;
                }
            }
        };

        let hi_var = self.get_by_id(&variable.id, tran).await?;

        Ok(hi_var)
    }

    pub async fn update_by_variable(&self, variable: &ApfRuVariable, update_time: NaiveDateTime, tran: &mut Transaction<'_, Postgres>) -> Result<()> {
        let mut hi_var = self.get_by_id(&variable.id, tran).await?;
        hi_var.var_type = variable.var_type.clone();
        hi_var.value = variable.value.clone();
        hi_var.name = variable.name.clone();
        hi_var.last_updated_time = update_time;

        self.update(&hi_var, tran).await?;

        Ok(())
    }

    pub async fn update(&self, obj: &ApfHiVarinst, tran: &mut Transaction<'_, Postgres>) -> Result<()> {
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

        let rst = sqlx::query(sql)
            .bind(&obj.var_type)
            .bind(&obj.name)
            .bind(&obj.value)
            .bind(&obj.execution_id)
            .bind(&obj.task_id)
            .bind(&obj.last_updated_time)
            .bind(&obj.id)
            .bind(&obj.rev)
            .execute(&mut *tran)
            .await?;

        if rst.rows_affected() != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError,
                    Some(
                        &format!(
                            "apf_hi_varinst({}) is not updated correctly, affects ({}) != 1", 
                            obj.id,
                            rst.rows_affected()
                        )
                    ),
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use sqlx::Acquire;
    use crate::boot::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::dao::apf_ru_variable_dao::tests::create_test_apf_ru_var;
    use crate::get_now;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::VarType;

    use super::*;

    #[tokio::test]
    async fn test_create_and_update() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;
        let task = create_test_task(&proc_inst, &mut tran).await;

        let mut ru_var = create_test_apf_ru_var(&task, &mut tran).await;
        assert_eq!(ru_var.proc_inst_id, task.proc_inst_id);

        let hi_var_dao = ApfHiVarinstDao::new();
        let hi_var = hi_var_dao.create_by_variable(&ru_var, get_now(), &mut tran).await.unwrap();
        assert_eq!(hi_var.id, ru_var.id);

        ru_var.var_type = VarType::STRING;
        let update_var = "test_update_hi_var";
        ru_var.value = update_var.to_owned();
        hi_var_dao.update_by_variable(&ru_var, get_now(), &mut tran).await.unwrap();
        let hi_var2 = hi_var_dao.get_by_id(&ru_var.id, &mut tran).await.unwrap();
        assert_eq!(hi_var2.id, ru_var.id);
        assert_eq!(hi_var2.value, update_var.to_owned());

        tran.rollback().await.unwrap();
    }

}
