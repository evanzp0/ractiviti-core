
use sqlx::{Postgres, Transaction};
use crate::manager::engine::{CompleteTaskCmd, ContinueProcessOperator,
                             CreateAndStartProcessInstanceCmd, CreateTaskCmd, OperateRst,
                             OperatorContext, TakeOutgoingFlowsOperator};
use color_eyre::Result;

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
    pub async fn execute<'a>(&self, operator_ctx: &mut OperatorContext,
        tran: &mut Transaction<'a, Postgres>) -> Result<OperateRst> 
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