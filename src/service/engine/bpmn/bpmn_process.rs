use std::collections::HashMap;
use crate::service::engine::{BpmnManager, NodeType};
use super::BpmnElement;
use color_eyre::Result;
use crate::error::{AppError, ErrorCode};

#[derive(Debug, Default)]
pub struct BpmnProcess {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub terminate_on_false: Option<String>,
    pub elements: Vec<BpmnElement>,
    pub element_map: HashMap<String, BpmnElement>,
    pub end_event_terminate_node:  Option<BpmnElement>,
}

impl BpmnProcess {
    pub fn new(id: String, name: Option<String>, description: Option<String>, terminate_on_false: Option<String>,) -> Self {
        Self {
            id,
            name,
            description,
            terminate_on_false,
            elements: vec![],
            element_map: HashMap::new(),
            end_event_terminate_node: Some(BpmnManager::create_end_event_terminate_node()),
        }
    }

    pub fn get_start_event(&self) -> Result<BpmnElement> {

        for item in &self.elements {
            if let BpmnElement::Node(node) = item {
                if node.get_node_type() == NodeType::StartEvent {
                    let n = node.clone();
                    return Ok(BpmnElement::Node(n));
                }
            }
        }

        Err(
            AppError::new(
                ErrorCode::NotFound, 
                Some("StartEvent is not found"), 
                concat!(file!(), ":", line!()), None
            )
        )?
    }

    pub fn end_event_terminate_node_ex(&self) -> Result<BpmnElement> {
        let rst = self.end_event_terminate_node.clone().ok_or(
            AppError::unexpected_error(concat!(file!(), ":", line!())))?;

        Ok(rst)
    }
}