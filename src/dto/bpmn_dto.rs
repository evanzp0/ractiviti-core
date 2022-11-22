use serde::{Serialize};

#[derive(Serialize)]
pub struct BpmnResultDto {
    pub bpmn_id: String,
    pub bpmn_key: String,
    pub bpmn_name: String,
    pub xml: Option<String>,
}
