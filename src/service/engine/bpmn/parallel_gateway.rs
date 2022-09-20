use super::{BpmnNode, NodeType};

#[derive(Debug, Default)]
pub struct ParallelGateway {
    pub id: String,
    pub description: Option<String>,
}

impl BpmnNode for ParallelGateway {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_node_type(&self) -> NodeType {
        NodeType::ParallelGateway
    }

    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}

impl ParallelGateway {
    pub fn new(id: String, description: Option<String>) -> Self {
        Self {
            id,
            description,
        }
    }
}