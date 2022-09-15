
use sqlx::{Postgres, Transaction};
use crate::manager::engine::{OperateRst, Operator, OperatorContext};
use color_eyre::Result;

pub struct OperatorExecutor {
}

impl OperatorExecutor {
    pub fn new() -> Self {
        Self {
        }
    }

    pub async fn run<'a>(&mut self, operator_ctx: &mut OperatorContext, tran: &mut Transaction<'a, Postgres>)
            -> Result<OperateRst> {
        let mut rst = OperateRst::default();

        while !operator_ctx.queue.read().unwrap().is_empty() {
            let operator = operator_ctx.queue.write().unwrap().remove(0);
            let procinst = operator.execute(operator_ctx, tran).await?;

            if procinst != OperateRst::default() {
                rst = procinst;
            }
        }

        Ok(rst)
    }

    pub async fn execute<'a>(&mut self, operator: Operator, operator_ctx: &mut OperatorContext,
        tran: &mut Transaction<'a, Postgres>) -> Result<OperateRst> 
    {
        operator_ctx.queue.write().unwrap().push(operator);
        let rst = self.run(operator_ctx, tran).await;

        rst
    }
}
