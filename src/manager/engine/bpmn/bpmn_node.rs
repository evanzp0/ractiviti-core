use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use crate::manager::engine::BpmnElement;
use super::{BpmnEdge, BpmnProcess};

#[derive(Debug, PartialEq)]
pub enum NodeType {
    StartEvent,
    EndEvent,
    UserTask,
    ServiceTask,
    ExclusiveGateway,
    ParallelGateway,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl NodeType {
    pub fn name(&self) -> String {
        match self {
            NodeType::StartEvent => {"StartEvent".to_owned()}
            NodeType::EndEvent => {"EndEvent".to_owned()}
            NodeType::UserTask => {"UserTask".to_owned()}
            NodeType::ServiceTask => {"ServiceTask".to_owned()}
            NodeType::ExclusiveGateway => {"ExclusiveGateway".to_owned()}
            NodeType::ParallelGateway => {"ParallelGateway".to_owned()}
        }
    }
}

pub trait BpmnNode : Debug {
    fn get_id(&self) -> String;

    fn get_node_type(&self) -> NodeType;

    fn get_name(&self) -> Option<String> {
        None
    }

    fn get_from_key(&self) -> Option<String> {
        None
    }

    fn get_description(&self) -> Option<String> {
        None
    }

    fn out_flows(&self, process: &BpmnProcess) -> Vec<Arc<dyn BpmnEdge>> {
        let mut rst = vec![];

        for el in &process.elements {
            if let BpmnElement::Edge(flow) = el {
                if flow.get_source() == self.get_id() {
                    rst.push(flow.clone());
                }
            }
        }

        rst
    }

    fn in_flows(&self, process: &BpmnProcess) -> Vec<Arc<dyn BpmnEdge>> {
        let mut rst = vec![];
        for el in &process.elements {
            if let BpmnElement::Edge(flow) = el {
                if flow.get_target() == self.get_id() {
                    rst.push(flow.clone());
                }
            }
        }

        rst
    }

    fn candidate_groups(&self) -> Arc<Vec<String>> {
        Arc::new(Vec::new())
    }

    fn candidate_users(&self) -> Arc<Vec<String>> {
        Arc::new(Vec::new())
    }
}
