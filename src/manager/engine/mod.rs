pub mod process_engine;
pub mod repository_service;
pub mod runtime_service;
pub mod history_service;
pub mod task_service;
pub mod deployment_builder;
pub mod bpmn_manager;
pub mod bpmn;
pub mod behavior;
pub mod js_engine;
pub mod type_wrapper;
pub mod query;

pub use process_engine::*;
pub use repository_service::*;
pub use runtime_service::*;
pub use history_service::*;
pub use task_service::*;
pub use deployment_builder::*;
pub use bpmn_manager::*;
pub use bpmn::*;
pub use behavior::operator_executor::*;
pub use behavior::operator::*;
pub use behavior::*;
pub use js_engine::*;
pub use type_wrapper::*;

pub fn get_default_process_engine() -> ProcessEngine {
    ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE)
}