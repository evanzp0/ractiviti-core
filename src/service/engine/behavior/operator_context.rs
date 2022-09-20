use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use color_eyre::Result;

use crate::ArcRw;
use crate::error::{AppError, ErrorCode};
use crate::service::engine::{BpmnProcess, Operator};
use crate::model::WrappedValue;

#[derive(Debug)]
pub struct OperatorContext {
    pub group_id: Option<String>,
    pub user_id: Option<String>,
    pub variables: HashMap<String, WrappedValue>,
    pub queue: ArcRw<Vec<Operator>>,
    pub bpmn_process: Option<Arc<BpmnProcess>>,
}

#[allow(unused)]
impl OperatorContext {
    pub fn default() -> Self {
        Self {
            group_id: None,
            user_id: None,
            variables: HashMap::new(),
            queue: Arc::new(RwLock::new(Vec::<Operator>::new())),
            bpmn_process: None,
        }
    }

    pub fn new(group_id: Option<String>, user_id: Option<String>, variables: HashMap<String, WrappedValue>) -> Self {
        Self {
            group_id,
            user_id,
            variables,
            queue: Arc::new(RwLock::new(Vec::<Operator>::new())),
            bpmn_process: None,
        }
    }

    pub fn bpmn_process_ex(&self) -> Result<Arc<BpmnProcess>> {
        let bpmn_process = self.bpmn_process
            .clone()
            .ok_or(
                AppError::new(
                    ErrorCode::NotFound,
                    Some("bpmn_process not found"),
                    concat!(file!(), ":", line!()),
                    None
                )
            )?;

        Ok(bpmn_process)
    }

    pub fn is_terminated(&self) -> Result<bool> {
        let mut rst = false;
        let tmp_terminate_varname = self.bpmn_process_ex()?.terminate_on_false.clone();

        if let Some(terminate_varname) = tmp_terminate_varname {
            let terminate_value = self.variables
                .get(&terminate_varname)
                .and_then(|v| Some(v.clone()));
            if let Some(WrappedValue::Bool(v)) = terminate_value {
                rst = (v == false);
            }
        }

        Ok(rst)
    }
}