use super::{BpmnNode, NodeType};

#[derive(Debug, Default)]
pub struct ExclusiveGateway {
    pub id: String,
    pub description: Option<String>,
}

impl BpmnNode for ExclusiveGateway {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_node_type(&self) -> NodeType {
        NodeType::ExclusiveGateway
    }

    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

}

impl ExclusiveGateway {
    pub fn new(id: String, description: Option<String>) -> Self {
        Self {
            id,
            description,
        }
    }
}