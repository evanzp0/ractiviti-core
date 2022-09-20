use std::sync::Arc;
use super::BpmnNode;
use super::BpmnEdge;

#[derive(Debug, Clone)]
pub enum BpmnElement {
    Edge(Arc<dyn BpmnEdge>),
    Node(Arc<dyn BpmnNode>),
}

impl BpmnElement {
    pub fn get_element_id(&self) -> String {
        match self {
            BpmnElement::Edge(el) => {
                el.get_id().to_owned()
            }
            BpmnElement::Node(el) => {
                el.get_id().to_owned()
            }
        }
    }

    pub fn get_element_type(&self) -> String {
        match self {
            BpmnElement::Edge(el) => {
                el.get_edge_type()
            }
            BpmnElement::Node(el) => {
                el.get_node_type().to_string()
            }
        }
    }

    pub fn get_element_name(&self) -> Option<String> {
        match self {
            BpmnElement::Edge(_) => {
                None
            }
            BpmnElement::Node(el) => {
                el.get_name()
            }
        }
    }

    pub fn get_description(&self) -> Option<String> {
        match self {
            BpmnElement::Edge(_) => {
                None
            }
            BpmnElement::Node(el) => {
                el.get_description()
            }
        }
    }

    pub fn get_from_key(&self) -> Option<String> {
        match self {
            BpmnElement::Edge(_) => {
                None
            }
            BpmnElement::Node(el) => {
                el.get_from_key()
            }
        }
    }

}

