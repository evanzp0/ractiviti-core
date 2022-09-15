use color_eyre::Result;
use sqlx::{Error, Postgres, Transaction};
use uuid::Uuid;
use crate::error::{AppError, ErrorCode};
use crate::model::{ApfRuVariable, ApfRuVariableDto};

pub struct ApfRuVariableDao {

}

impl ApfRuVariableDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, obj: &ApfRuVariableDto, tran: &mut Transaction<'_, Postgres>)
                            -> Result<ApfRuVariable> {
        let sql = "insert into apf_ru_variable \
                (rev, var_type, name, value, proc_inst_id, execution_id, task_id) \
            values \
                (1, $1, $2, $3, $4, $5, $6) \
            returning * ";

        let rst = sqlx::query_as::<_, ApfRuVariable>(sql)
            .bind(&obj.var_type)
            .bind(&obj.name)
            .bind(&obj.value)
            .bind(&obj.proc_inst_id)
            .bind(&obj.execution_id)
            .bind(&obj.task_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn create_or_update(&self, obj: &ApfRuVariableDto, tran: &mut Transaction<'_, Postgres>)
                                      -> Result<ApfRuVariable> {
        let mut var_id = Uuid::default();
        let mut rst_variable = self.get_by_proc_inst(&obj.proc_inst_id, &obj.name, tran).await;
        match &mut rst_variable {
            Ok(variable) => {
                // do update
                variable.name = obj.name.clone();
                variable.value = obj.value.clone();
                variable.var_type = obj.var_type.clone();
                variable.execution_id = obj.execution_id.clone();
                variable.task_id = obj.task_id.clone();

                self.update(&variable, tran).await?;
                var_id = variable.id;
            },
            Err(error) => {
                // ptln!(error.is::<Error>());
                let err = error.downcast_ref::<Error>().ok_or(
                    AppError::unexpected_error(concat!(file!(), ":", line!())))?;
                if let Error::RowNotFound = err {
                    // do create
                    let rst = self.create(obj, tran).await?;

                    var_id = rst.id;
                }
            }
        }

        let rst = self.get_by_id(&var_id, tran).await?;

        Ok(rst)
    }

    pub async fn update(&self, obj: &ApfRuVariable, tran: &mut Transaction<'_, Postgres>)
                            -> Result<()> {
        let sql = "update apf_ru_variable \
                        set rev = rev + 1, \
                            var_type = $1, \
                            name = $2, \
                            value = $3, \
                            execution_id = $4, \
                            task_id = $5 \
                        where id = $6 \
                          and rev = $7";

        let rst = sqlx::query(sql)
            .bind(&obj.var_type)
            .bind(&obj.name)
            .bind(&obj.value)
            .bind(&obj.execution_id)
            .bind(&obj.task_id)
            .bind(&obj.id)
            .bind(&obj.rev)
            .execute(&mut *tran)
            .await?;

        if rst.rows_affected() != 1 {
            Err(AppError::new(ErrorCode::InternalError,
                              Some(&format!("apf_ru_variable({}) is not updated correctly, affects ({}) != 1",
                                            obj.id, rst.rows_affected())),
                              concat!(file!(), ":", line!()),
                              None))?
        }

        Ok(())
    }

    pub async fn get_by_id(&self, id: &Uuid, tran: &mut Transaction<'_, Postgres>)
                               -> Result<ApfRuVariable> {
        let sql = "select id, rev, var_type, name, value, proc_inst_id, execution_id, task_id \
                        from apf_ru_variable \
                        where id = $1";

        let rst = sqlx::query_as::<_, ApfRuVariable>(sql)
            .bind(id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn get_by_proc_inst(&self, proc_inst_id: &Uuid, name: &str,
                                      tran: &mut Transaction<'_, Postgres>)
                               -> Result<ApfRuVariable> {
        let sql = "select id, rev, var_type, name, value, proc_inst_id, execution_id, task_id \
                        from apf_ru_variable \
                        where proc_inst_id = $1 \
                          and name = $2";

        let rst = sqlx::query_as::<_, ApfRuVariable>(sql)
            .bind(proc_inst_id)
            .bind(name)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn find_all_by_proc_inst(&self, proc_inst_id: &Uuid,
                                      tran: &mut Transaction<'_, Postgres>)
                                      -> Result<Vec<ApfRuVariable>> {
        let sql = "select id, rev, var_type, name, value, proc_inst_id, execution_id, task_id \
                        from apf_ru_variable \
                        where proc_inst_id = $1";

        let rst = sqlx::query_as::<_, ApfRuVariable>(sql)
            .bind(proc_inst_id)
            .fetch_all(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn delete_by_proc_inst_id(&self, proc_inst_id: &Uuid, tran: &mut Transaction<'_, Postgres>)
                                       -> Result<u64> {
        let sql = "delete from apf_ru_variable \
                        where proc_inst_id = $1";
        let rst = sqlx::query(sql)
            .bind(proc_inst_id)
            .execute(&mut *tran)
            .await?;

        Ok(rst.rows_affected())
    }
}

#[cfg(test)]
pub mod tests {
    use sqlx::Acquire;
    use crate::boot::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::{ApfRuTask, VarType};

    use super::*;

    #[actix_rt::test]
    async fn test_create_or_update() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;
        let task = create_test_task(&proc_inst, &mut tran).await;

        let ru_var = create_test_apf_ru_var(&task, &mut tran).await;
        assert_eq!(ru_var.proc_inst_id, task.proc_inst_id);

        let mut dto_1 = ApfRuVariableDto::default();
        dto_1.name= "test_create_or_update".to_owned();
        dto_1.value= "test_create_value".to_owned();
        dto_1.proc_inst_id = ru_var.proc_inst_id;

        let ru_var_dao = ApfRuVariableDao::new();
        let ru_var_1 = ru_var_dao.create_or_update(&dto_1, &mut tran).await.unwrap();
        assert_ne!(ru_var_1.id, ApfRuVariable::default().id);
        assert_eq!(ru_var_1.name , "test_create_or_update".to_owned());
        assert_eq!(ru_var_1.value , "test_create_value".to_owned());

        dto_1.value= "test_update_value".to_owned();
        let ru_var_2 = ru_var_dao.create_or_update(&dto_1, &mut tran).await.unwrap();
        assert_eq!(ru_var_2.id, ru_var_1.id);
        assert_eq!(ru_var_2.name , "test_create_or_update".to_owned());
        assert_eq!(ru_var_2.value , "test_update_value".to_owned());

        tran.rollback().await.unwrap();
    }

    #[actix_rt::test]
    async fn test_create_and_delete() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;
        let task = create_test_task(&proc_inst, &mut tran).await;

        let ru_var = create_test_apf_ru_var(&task, &mut tran).await;
        assert_eq!(ru_var.proc_inst_id, task.proc_inst_id);

        let ru_var_dao = ApfRuVariableDao::new();
        let rst = ru_var_dao.delete_by_proc_inst_id(&ru_var.proc_inst_id, &mut tran).await.unwrap();
        assert_eq!(rst, 1);

        let mut dto = ApfRuVariableDto::default();
        dto.name= "null".to_owned();
        dto.proc_inst_id = ru_var.proc_inst_id;

        ru_var_dao.create_or_update(&dto, &mut tran).await.unwrap();

        tran.rollback().await.unwrap();
    }

    pub async fn create_test_apf_ru_var(task: &ApfRuTask, tran: &mut Transaction<'_, Postgres>) -> ApfRuVariable {

        let obj = ApfRuVariableDto {
            var_type: VarType::STRING,
            name: "approval".to_string(),
            value: "true".to_string(),
            proc_inst_id: task.proc_inst_id,
            execution_id: Some(task.execution_id.clone()),
            task_id: Some(task.id.clone()),
        };

        let ru_var_dao = ApfRuVariableDao::new();
        let rst = ru_var_dao.create(&obj, tran).await.unwrap();

        rst
    }
}

