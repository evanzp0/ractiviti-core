use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use tokio_postgres::Transaction;
use color_eyre::Result;

use crate::{get_now, RcRefCell};
use crate::error::{AppError, ErrorCode};
use crate::service::engine::{ BpmnEdge, BpmnElement, NodeType, OperateRst, Operator,OperatorContext, TakeOutgoingFlowsOperator };
use crate::model::{ApfRuExecution, ApfRuTask, ApfRuVariableDto, NewApfHiActinst, NewApfRuExecution, WrappedValue};
use crate::dao::{ApfHiActinstDao, ApfHiVarinstDao, ApfRuExecutionDao, ApfRuVariableDao};
use crate::service::engine::query::TaskQuery;

#[derive(Debug)]
pub struct BaseOperator {
    pub proc_inst: Rc<ApfRuExecution>,
    current_exec: Option<RcRefCell<ApfRuExecution>>,
    pub element: BpmnElement,
    pub terminate_element: Option<BpmnElement>,
    pub current_task: Option<Rc<ApfRuTask>>,
}

impl BaseOperator {
    pub fn new(
        proc_inst: Rc<ApfRuExecution>, 
        current_exec: Option<RcRefCell<ApfRuExecution>>, 
        element: BpmnElement, 
        terminate_element: Option<BpmnElement>, 
        current_task: Option<Rc<ApfRuTask>>
    ) -> Self {
        Self {
            proc_inst,
            current_exec,
            element,
            terminate_element,
            current_task,
        }
    }

    pub fn current_exec(&self) -> Option<RcRefCell<ApfRuExecution>> {
        self.current_exec.clone()
    }

    pub fn set_current_exec(&mut self, current_exec: RcRefCell<ApfRuExecution>) {
        self.current_exec = Some(current_exec);
    }

    pub fn current_task_ex(&self) -> Result<Rc<ApfRuTask>> {
        let current_task = self.current_task
            .clone()
            .ok_or(AppError::new(ErrorCode::NotFound, Some("current task not found"), concat!(file!(), ":", line!()), None))?;
        Ok(current_task)
    }

    pub fn current_excution_ex(&self) -> Result<RcRefCell<ApfRuExecution>> {
        let current_exec = self.current_exec
            .clone()
            .ok_or(AppError::new(ErrorCode::NotFound, Some("current execution not found"), concat!(file!(), ":", line!()), None))?;
        Ok(current_exec)
    }

    pub async fn continue_outflow(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>)
            -> Result<()> {
        match &self.element {
            BpmnElement::Edge(_) => {
                Err(
                    AppError::new(
                        ErrorCode::NotFound,
                        Some("source bpmn node not found when taking outgoing flows"), 
                        concat!(file!(), ":", line!()), 
                        None
                    )
                )?
            },
            BpmnElement::Node(node) => {
                let bpmn_process = operator_ctx.bpmn_process_ex()?;
                let out_flows = node.out_flows(bpmn_process.as_ref());

                match node.get_node_type() {
                    NodeType::StartEvent => {
                        self._continue_outflow(operator_ctx, &out_flows, tran).await?;
                    },
                    NodeType::EndEvent => {},
                    NodeType::UserTask => {
                        self._continue_outflow(operator_ctx, &out_flows, tran).await?;
                    },
                    NodeType::ServiceTask => {
                        self._continue_outflow(operator_ctx, &out_flows, tran).await?;
                    },
                    NodeType::ExclusiveGateway => {
                        Err(
                            AppError::new(
                                ErrorCode::NotSupportError,
                                Some("ExclusiveGateway node is not supported by continue_outflow()"),
                                concat!(file!(), ":", line!()),
                                None
                            )
                        )?
                    },
                    NodeType::ParallelGateway => {
                        Err(
                            AppError::new(
                                ErrorCode::NotFound,
                                    Some("ParallelGateway is not supported by continue_outflow()"),
                                    concat!(file!(), ":", line!()),
                                    None
                                )
                        )?
                    }
                }
            },
        }

        Ok(())
    }

    pub async fn mark_begin_exection(
        &self, 
        element_id: &str, 
        start_user: Option<String>, 
        start_time: i64, 
        tran: &Transaction<'_>
    ) -> Result<()> {
        let current_exec = self.current_excution_ex()?;
        let mut current_exec = current_exec.borrow_mut();
        let current_exec_id = current_exec.id.clone();
        current_exec.element_id = Some(element_id.to_owned());
        current_exec.start_user = start_user.clone();
        current_exec.start_time = start_time;

        let exec_dao = ApfRuExecutionDao::new(tran);
        exec_dao.mark_begin(&current_exec_id, element_id, start_user, start_time).await?;

        Ok(())
    }

    pub async fn mark_end_execution<'a>(&self, operator_ctx: &OperatorContext, tran: &Transaction<'_>) -> Result<()> {
        let current_exec = self.current_excution_ex()?;
        let current_exec = current_exec.borrow();
        let current_exec_id = current_exec.id.clone();
        let element_id = current_exec.element_id()?;
        let end_time = get_now();
        let end_user_id = operator_ctx.user_id.clone();

        let hi_act_dao = ApfHiActinstDao::new(tran);
        hi_act_dao.mark_end(&current_exec_id, &element_id, end_time, end_user_id).await?;

        Ok(())
    }

    async fn _continue_outflow(
        &self, 
        operator_ctx: &mut OperatorContext, 
        out_flows: &Vec<Arc<dyn BpmnEdge>>, 
        tran: &Transaction<'_>
    ) -> Result<()> {
        let current_exec = self.current_excution_ex()?;
        let out_flow = out_flows
            .get(0)
            .ok_or(AppError::new(ErrorCode::NotFound, Some("start event must have 1 outflow"), concat!(file!(), ":", line!()), None))?;
        let element = BpmnElement::Edge(out_flow.clone());
        let element_id = out_flow.get_id();

        // set edge for current exection and update the start of it
        self.mark_begin_exection(&element_id, operator_ctx.user_id.clone(), get_now(), tran).await?;

        // continue to handle the edge
        let next_operator = TakeOutgoingFlowsOperator::new(element, self.proc_inst.clone(), Some(current_exec));
        operator_ctx.queue.push(Operator::TakeOutgoingFlowsOperator(next_operator));

        Ok(())
    }

    pub async fn create_hi_actinst(&self, task_id: Option<String>, tran: &Transaction<'_>) -> Result<OperateRst> {
        let current_exec = self.current_excution_ex()?;
        let current_exec = current_exec.borrow();
        let new_hi_actinst = NewApfHiActinst {
            rev: 1,
            proc_def_id: self.proc_inst.proc_def_id.clone(),
            proc_inst_id: self.proc_inst.proc_inst_id.clone(),
            execution_id: current_exec.id.clone(),
            element_id: Some(self.element.get_element_id()),
            element_name: self.element.get_element_name(),
            element_type: Some(self.element.get_element_type()),
            start_time: get_now(),
            start_user_id: current_exec.start_user.clone(),
            task_id,
            end_time: None,
            duration: None
        };
        let hi_act_dao = ApfHiActinstDao::new(tran);
        let _rst = hi_act_dao.create(&new_hi_actinst).await?;

        Ok(OperateRst::default())
    }

    pub async fn create_current_execution<'a>(
        &self, 
        element_id: &str, 
        start_time: i64, 
        start_user: Option<String>, 
        tran: &Transaction<'_>
    ) -> Result<RcRefCell<ApfRuExecution>> {
        let new_exec = NewApfRuExecution {
            parent_id: Some(self.proc_inst.id.clone()),
            proc_inst_id: Some(self.proc_inst.id.clone()),
            proc_def_id: self.proc_inst.proc_def_id.clone(),
            root_proc_inst_id: Some(self.proc_inst.id.clone()),
            element_id: Some(element_id.to_owned()),
            is_active: 1,
            start_time: start_time,
            start_user,
            business_key: self.proc_inst.business_key.clone(),
        };
        
        let exec_dao = ApfRuExecutionDao::new(tran);
        let current_execution = exec_dao.create(&new_exec).await?;

        Ok(Rc::new(RefCell::new(current_execution)))
    }

    pub async fn create_or_update_variables(&self, variables: &mut HashMap<String, WrappedValue>, tran: &Transaction<'_>) -> Result<()> {
        for (key, value) in variables.iter() {
            let mut dto = ApfRuVariableDto::default();
            dto.var_type = value.get_type();
            dto.value =  value.as_str();
            dto.name = key.to_owned();
            dto.proc_inst_id = self.proc_inst.id.clone();

            if let Some(current_exec) = &self.current_exec {
                dto.execution_id = Some(current_exec.borrow().id.clone());
            }

            if let Some(task) = &self.current_task {
                dto.task_id = Some(task.id.clone());
            }

            let ru_var_dao = ApfRuVariableDao::new(tran);
            let ru_var = ru_var_dao.create_or_update(&dto).await?;

            let hi_var_dao = ApfHiVarinstDao::new(tran);
            hi_var_dao.create_or_update_by_variable(&ru_var, get_now()).await?;
        }

        Ok(())
    }


    pub async fn check_complete_task_priviledge<'a>(
        &self, 
        task: Rc<ApfRuTask>, 
        element: &BpmnElement,
        operator_ctx: &mut OperatorContext,
        tran: &Transaction<'_>
    ) -> Result<()> {
        // check the current user has priviledge complete the task
        let mut task_query = TaskQuery::new(tran).id(&task.id);
        if let BpmnElement::Node(node) = element {
            if node.candidate_users().len() > 0 {
                match node.get_node_type() {
                    NodeType::UserTask => {
                        if let Some(_) = &operator_ctx.user_id {
                            task_query = task_query.candidate_user(operator_ctx.user_id.clone());
                        }
                    },
                    NodeType::ServiceTask => {
                        if let Some(_) = &operator_ctx.user_id {
                            task_query = task_query.candidate_user(operator_ctx.user_id.clone());
                        }
                    },
                    _ => {},
                }
            }

            if node.candidate_groups().len() > 0 {
                match node.get_node_type() {
                    NodeType::UserTask => {
                        if let Some(_) = &operator_ctx.group_id {
                            task_query = task_query.candidate_user(operator_ctx.group_id.clone());
                        }
                    },
                    NodeType::ServiceTask => {
                        if let Some(_) = &operator_ctx.group_id {
                            task_query = task_query.candidate_user(operator_ctx.group_id.clone());
                        }
                    },
                    _ => {},
                }
            }
        }

        let tasks = task_query.fetch_all().await?;

        if tasks.is_empty() {
            Err(
                AppError::new(
                    ErrorCode::UnAuthorized,
                    Some(
                        &format!(
                            "Current user ({}) has not been authorized to complete the task ({})",
                            operator_ctx.user_id.clone().unwrap_or("?".to_owned()), 
                            task.id
                        )
                    ),
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?;
        }

        Ok(())
    }

}