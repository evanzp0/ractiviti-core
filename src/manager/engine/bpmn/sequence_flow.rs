use std::sync::Arc;
use crate::manager::engine::BpmnElement;
use super::{BpmnEdge, BpmnProcess, BpmnNode};

#[derive(Debug, Default)]
pub struct SequenceFlow {
    pub id: String,
    pub source_ref: String,
    pub target_ref: String,
    pub condition_expression: Option<String>,
}

impl BpmnEdge for SequenceFlow {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_source(&self) -> String {
        self.source_ref.to_owned()
    }

    fn get_target(&self) -> String {
        self.target_ref.to_owned()
    }

    fn get_condition_expr(&self) -> Option<String> {
        self.condition_expression.clone()
    }

    fn from_node(&self, process: &BpmnProcess) -> Option<Arc<dyn BpmnNode>> {
        let element_map = &process.element_map;
        let element = element_map.get(&self.source_ref);
        let mut rst: Option<Arc<dyn BpmnNode>> = None;

        if let Some(el) = element {
            if let BpmnElement::Node(node) = el {
                rst = Some(node.clone())
            }
        }

        rst
    }

    fn to_node(&self, process: &BpmnProcess) -> Option<Arc<dyn BpmnNode>> {
        // let node_map = &process.node_map;
        // let node = node_map.get(&self.target_ref)
        //     .and_then(|value| Some((*value).clone()));
        // node

        let element_map = &process.element_map;
        let element = element_map.get(&self.target_ref);
        let mut rst: Option<Arc<dyn BpmnNode>> = None;

        if let Some(el) = element {
            if let BpmnElement::Node(node) = el {
                rst = Some(node.clone())
            }
        }

        rst
    }
}

impl SequenceFlow {
    pub fn new(id: String, source_ref: String, target_ref: String, condition_expression: Option<String>)
            -> Self {
        Self {
            id,
            source_ref,
            target_ref,
            condition_expression,
        }
    }
}