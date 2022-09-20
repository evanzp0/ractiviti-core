use color_eyre::Result;
use tokio_postgres::Transaction;

use crate::service::engine::{
    CompleteTaskCmd, ContinueProcessOperator, CreateAndStartProcessInstanceCmd, CreateTaskCmd, OperateRst,
    OperatorContext, TakeOutgoingFlowsOperator
};

#[derive(Debug)]
pub enum Operator {
    CreateAndStartProcessInstanceCmd(CreateAndStartProcessInstanceCmd),
    ContinueProcessOperator(ContinueProcessOperator),
    TakeOutgoingFlowsOperator(TakeOutgoingFlowsOperator),
    CreateTaskCmd(CreateTaskCmd),
    CompleteTaskCmd(CompleteTaskCmd),
}

unsafe impl Send for Operator{}

impl Operator {
    pub async fn execute(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<OperateRst> 
    {
        match self {
            Operator::CreateAndStartProcessInstanceCmd(opt) => {
                opt.execute(operator_ctx, tran).await
            },
            Operator::ContinueProcessOperator(opt) => {
                opt.execute(operator_ctx, tran).await
            },
            Operator::CompleteTaskCmd(opt) => {
                opt.execute(operator_ctx, tran).await
            },
            Operator::CreateTaskCmd(opt) => {
                opt.execute(operator_ctx, tran).await
            },
            Operator::TakeOutgoingFlowsOperator(opt) => {
                opt.execute(operator_ctx, tran).await
            },
        }
    }
}