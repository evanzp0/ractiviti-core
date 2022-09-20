use super::{BpmnNode, NodeType};

#[derive(Debug, Default)]
pub struct EndEvent {
    pub id: String,
    pub description: Option<String>,

}

impl BpmnNode for EndEvent {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_node_type(&self) -> NodeType {
        NodeType::EndEvent
    }

    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}

impl EndEvent {
    pub fn new(id: String, description: Option<String>) -> Self {
        Self {
            id,
            description,
        }
    }
}