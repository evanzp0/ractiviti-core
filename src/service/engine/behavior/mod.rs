pub mod operator_executor;
pub mod operator;
pub mod continue_process_operator;
pub mod take_outgoing_flows_operator;
pub mod create_and_start_process_instance_cmd;
pub mod operator_context;
pub mod start_event_behavior;
pub mod base_operator;
pub mod create_task_cmd;
pub mod complete_task_cmd;
pub mod service_task_behavior;
pub mod user_task_behavior;
pub mod exclusive_gateway_behavior;
pub mod parallel_gateway_behavior;
pub mod end_event_behavior;

use std::sync::Arc;
pub use operator::*;
pub use operator_executor::*;
pub use continue_process_operator::*;
pub use take_outgoing_flows_operator::*;
pub use create_and_start_process_instance_cmd::*;
pub use operator_context::*;
pub use start_event_behavior::*;
use crate::model::ApfRuExecution;
pub use base_operator::*;
pub use create_task_cmd::*;
pub use complete_task_cmd::*;
pub use service_task_behavior::*;
pub use user_task_behavior::*;
pub use exclusive_gateway_behavior::*;
pub use parallel_gateway_behavior::*;
pub use end_event_behavior::*;

#[derive(Default, PartialEq, Debug)]
pub struct OperateRst {
    pub process_instantce: Option<Arc<ApfRuExecution>>
}