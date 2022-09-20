use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;

use crate::{gen_id};
use crate::error::{AppError, ErrorCode};
use crate::model::{ApfRuVariable, ApfRuVariableDto};

use super::{BaseDao, Dao};

pub struct ApfRuVariableDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfRuVariableDao<'a> {

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfRuVariableDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }
    
    pub async fn create(&self, obj: &ApfRuVariableDto) -> Result<ApfRuVariable> {
        let sql = r#"
            insert into apf_ru_variable (
                rev, var_type, name, value, proc_inst_id, 
                execution_id, task_id, id
            ) values (
                $1, $2, $3, $4, $5, 
                $6, $7, $8
            )
            returning *
        "#;
        let new_id = gen_id();
        let rev:i32 = 1;
        let stmt = self.tran().prepare(sql).await?;
        let row = self
            .tran()
            .query_one(
                &stmt, 
                &[
                    &rev,
                    &obj.var_type,
                    &obj.name,
                    &obj.value,
                    &obj.proc_inst_id,
                    &obj.execution_id,
                    &obj.task_id,
                    &new_id,
                ]
            )
            .await?;
        let rst = ApfRuVariable::from_row(row)?;

        Ok(rst)
    }

    pub async fn create_or_update(&self, obj: &ApfRuVariableDto) -> Result<ApfRuVariable> {
        let mut var_id = "".to_owned();
        let mut rst_variable = self.get_by_proc_inst(&obj.proc_inst_id, &obj.name).await;
        match &mut rst_variable {
            Ok(variable) => {
                // do update
                variable.name = obj.name.clone();
                variable.value = obj.value.clone();
                variable.var_type = obj.var_type.clone();
                variable.execution_id = obj.execution_id.clone();
                variable.task_id = obj.task_id.clone();

                self.update(&variable).await?;
                var_id = variable.id.clone();
            },
            Err(error) => {
                let err = error.downcast_ref::<AppError>()
                    .ok_or(AppError::internal_error(&format!("{} : {} , {}", file!(), line!(), error.to_string())))?; 

                if ErrorCode::NotFound == err.code {
                    // do create
                    let rst = self.create(obj).await?;

                    var_id = rst.id;
                }
            }
        }

        let rst = self.get_by_id(&var_id).await?;

        Ok(rst)
    }

    pub async fn update(&self, obj: &ApfRuVariable) -> Result<u64> {
        let sql = r#"
            update apf_ru_variable
            set rev = rev + 1,
                var_type = $1,
                name = $2,
                value = $3,
                execution_id = $4,
                task_id = $5
            where id = $6
                and rev = $7
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
                    &obj.id,
                    &obj.rev,
                ]
            )
            .await?;

        if r != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError, 
                    Some(&format!("apf_ru_variable({}) is not updated correctly, affects ({}) != 1", obj.id, r)), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(r)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<ApfRuVariable> {
        let sql = r#"
            select id, rev, var_type, name, value, 
                proc_inst_id, execution_id, task_id 
            from apf_ru_variable
            where id = $1
        "#;

        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&id]).await?;
        let rst = ApfRuVariable::from_row(row)?;

        Ok(rst)
    }

    pub async fn get_by_proc_inst(&self, proc_inst_id: &str, name: &str) -> Result<ApfRuVariable> {
        let sql = r#"
            select id, rev, var_type, name, value, 
                proc_inst_id, execution_id, task_id 
            from apf_ru_variable 
            where proc_inst_id = $1 
                and name = $2
            limit 2
        "#;

        let stmt = self.tran().prepare(sql).await?;
        let rows = self.tran().query(&stmt, &[&proc_inst_id, &name]).await?;
        if rows.len() == 0 {
            Err(
                AppError::new(
                    ErrorCode::NotFound, 
                    Some(&format!("apf_ru_variable(proc_inst_id:{}, name:{}) is not exist", proc_inst_id, name)), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        let rst = ApfRuVariable::from_row_ref(&rows[0])?;

        Ok(rst)
    }

    pub async fn find_all_by_proc_inst(&self, proc_inst_id: &str) -> Result<Vec<ApfRuVariable>> {
        let sql = r#"
            select id, rev, var_type, name, value, 
                proc_inst_id, execution_id, task_id
            from apf_ru_variable
            where proc_inst_id = $1
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let rows = self.tran().query(&stmt, &[&proc_inst_id]).await?;
        let rst = rows
            .iter()
            .map(|row| ApfRuVariable::from_row_ref(row).expect("unexpected_error"))
            .collect::<Vec<ApfRuVariable>>();

        Ok(rst)
    }

    pub async fn delete_by_proc_inst_id(&self, proc_inst_id: &str) -> Result<u64> {
        let sql = r#"delete from apf_ru_variable where proc_inst_id = $1"#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran().execute(&stmt, &[&proc_inst_id]).await?;

        Ok(r)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::common::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::service::engine::tests::create_test_deploy;
    use crate::model::{ApfRuTask, WrappedValue};

    use super::*;

    #[tokio::test]
    async fn test_create_or_update() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let proc_inst = create_test_procinst(&procdef, &tran).await;
        let task = create_test_task(&proc_inst, &tran).await;

        let ru_var = create_test_apf_ru_var(&task, &tran).await;
        assert_eq!(ru_var.proc_inst_id, task.proc_inst_id);

        let mut dto_1 = ApfRuVariableDto::default();
        dto_1.name= "test_create_or_update".to_owned();
        dto_1.value= "test_create_value".to_owned();
        dto_1.proc_inst_id = ru_var.proc_inst_id;

        let ru_var_dao = ApfRuVariableDao::new(&tran);
        let ru_var_1 = ru_var_dao.create_or_update(&dto_1).await.unwrap();
        assert_ne!(ru_var_1.id, ApfRuVariable::default().id);
        assert_eq!(ru_var_1.name , "test_create_or_update".to_owned());
        assert_eq!(ru_var_1.value , "test_create_value".to_owned());

        dto_1.value= "test_update_value".to_owned();
        let ru_var_2 = ru_var_dao.create_or_update(&dto_1).await.unwrap();
        assert_eq!(ru_var_2.id, ru_var_1.id);
        assert_eq!(ru_var_2.name , "test_create_or_update".to_owned());
        assert_eq!(ru_var_2.value , "test_update_value".to_owned());

        tran.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_create_and_delete() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let proc_inst = create_test_procinst(&procdef, &tran).await;
        let task = create_test_task(&proc_inst, &tran).await;

        let ru_var = create_test_apf_ru_var(&task, &tran).await;
        assert_eq!(ru_var.proc_inst_id, task.proc_inst_id);

        let ru_var_dao = ApfRuVariableDao::new(&tran);
        let rst = ru_var_dao.delete_by_proc_inst_id(&ru_var.proc_inst_id).await.unwrap();
        assert_eq!(rst, 1);

        let mut dto = ApfRuVariableDto::default();
        dto.name= "null".to_owned();
        dto.proc_inst_id = ru_var.proc_inst_id;

        ru_var_dao.create_or_update(&dto).await.unwrap();

        tran.rollback().await.unwrap();
    }

    pub async fn create_test_apf_ru_var(task: &ApfRuTask, tran: &Transaction<'_>) -> ApfRuVariable {
        let value = WrappedValue::Str("true".to_owned());

        let obj = ApfRuVariableDto {
            var_type: value.get_type(),
            name: "approval".to_string(),
            value: value.as_str(),
            proc_inst_id: task.proc_inst_id.clone(),
            execution_id: Some(task.execution_id.clone()),
            task_id: Some(task.id.clone()),
        };

        let ru_var_dao = ApfRuVariableDao::new(tran);
        let rst = ru_var_dao.create(&obj).await.unwrap();

        rst
    }
}

