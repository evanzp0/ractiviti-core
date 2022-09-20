
use super::{BpmnNode, NodeType};

#[derive(Debug, Default)]
pub struct StartEvent {
    pub id: String,
    pub description: Option<String>,
}

impl BpmnNode for StartEvent {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_node_type(&self) -> NodeType {
        NodeType::StartEvent
    }

    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}

impl StartEvent {
    pub fn new(id: String, description: Option<String>) -> Self {
        Self {
            id,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use log4rs::debug;
    use crate::service::engine::{BpmnElement, BpmnManager, DeploymentBuilder};

    #[test]
    fn test_parse() {
        let builder = DeploymentBuilder::new();
        let deploy_builder = builder.add_file("bpmn/process1.bpmn.xml").unwrap();

        let bpmn_manager = BpmnManager::new();
        let bpmn_xml = String::from_utf8(deploy_builder
            .new_deployment.new_bytearray.bytes.clone()
            .unwrap_or(Vec::new())).unwrap();
        let bpmn_def = bpmn_manager.parse(bpmn_xml).unwrap();

        for (key, element) in &bpmn_def.process.element_map {
            if let BpmnElement::Node(node) = element {
                let out_flows = node.out_flows(&bpmn_def.process);
                let in_flows = node.in_flows(&bpmn_def.process);
                debug!("key: {}, in: {:?}, out: {:?}", key, in_flows, out_flows);
            }
        }
    }
}