use std::fmt::Debug;
use std::sync::Arc;
use super::{BpmnNode, BpmnProcess};

pub trait BpmnEdge : Debug + Send + Sync {
    fn get_id(&self) -> String;

    fn get_source(&self) -> String;

    fn get_target(&self) -> String;

    fn get_condition_expr(&self) -> Option<String> {
        None
    }

    fn from_node(&self, process: &BpmnProcess) -> Option<Arc<dyn BpmnNode>>;

    fn to_node(&self, process: &BpmnProcess) ->  Option<Arc<dyn BpmnNode>>;

    fn get_edge_type(&self) -> String {
        "SequenceFlow".to_owned()
    }
}
