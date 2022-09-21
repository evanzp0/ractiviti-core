use std::collections::HashMap;
use std::sync::Arc;
use color_eyre::Result;
use log4rs_macros::error;
use xml_doc_log4rs::Document;
use crate::error::{AppError, ErrorCode};
use super::{StartEvent, BpmnElement,
    BpmnProcess, EndEvent, UserTask,
    ServiceTask, SequenceFlow, BpmnNode,
    BpmnEdge, ExclusiveGateway, BpmnDefinitions,
    ParallelGateway};

pub struct BpmnManager {}

impl BpmnManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_from_bytes(&self, arr: Vec<u8>) -> Result<BpmnDefinitions> {
        let bpmn_xml = String::from_utf8(arr)?;
        let bpmn_def = BpmnManager::new().parse(bpmn_xml)?;

        Ok(bpmn_def)
    }

    pub fn parse(&self, xml: String) -> Result<BpmnDefinitions> {
        let doc = Document::parse_str(&xml)
            .map_err(|err| {
                error!("{:?}", err);
                AppError::new(
                    ErrorCode::ParseError,
                    Some("BPMN 文件解析错误"),
                    concat!(file!(), ":", line!()),
                    None
                )
            })?;

        let root_el = doc.root_element()
            .ok_or(AppError::new(ErrorCode::ParseError, Some("BPMN 文件格式错误，缺少根节点"), concat!(file!(), ":", line!()), None))?;
        if root_el.name(&doc) != "definitions" {
            Err(AppError::new(ErrorCode::ParseError, Some("BPMN 文件格式错误，缺少 definitions 节点"), concat!(file!(), ":", line!()), None))?;
        }

        let proc_el = root_el.find(&doc, "process")
            .ok_or(AppError::new(ErrorCode::ParseError, Some("BPMN 文件格式错误，缺少 process 节点"), concat!(file!(), ":", line!()), None))?;

        let proc_id = proc_el.attribute(&doc, "id")
            .ok_or(AppError::new(ErrorCode::ParseError, Some("BPMN 文件格式错误，process 节点缺少 id 属性"), concat!(file!(), ":", line!()), None))?;
        let proc_name = proc_el.attribute(&doc, "name")
            .and_then(|s| Some(s.to_owned()));

        let proc_description = proc_el.attribute(&doc, "description")
            .and_then(|s| Some(s.to_owned()));

        let terminate_on_false = proc_el.attribute(&doc, "terminate_on_false")
            .and_then(|s| Some(s.to_owned()));

        let bpmn_proc = BpmnProcess::new(
            proc_id.to_owned(),
            proc_name,
            proc_description,
            terminate_on_false,
        );

        let mut bpmn_def = BpmnDefinitions::new(xml, bpmn_proc);

        for child_el in proc_el.child_elements(&doc) {
            let id = child_el.attribute(&doc, "id").ok_or(
                AppError::new(ErrorCode::ParseError, Some(&format!("Bpmn element 缺少 id 属性")), concat!(file!(), ":", line!()), None))?;
            let el_name = child_el.name(&doc).to_string();
            let description = child_el.attribute(&doc, "description")
                .and_then(|s| Some(s.to_owned()));

            let pe_elements = &mut bpmn_def.process.elements;
            let element_map = &mut bpmn_def.process.element_map;

            if el_name == "startEvent" {
                let node = Arc::new(StartEvent::new(id.to_owned(), description.clone()));
                Self::add_node(id, node, pe_elements, element_map)?;
            } else if el_name == "endEvent" {
                let node = Arc::new(EndEvent::new(id.to_owned(), description.clone()));
                Self::add_node(id, node, pe_elements, element_map)?;
            } else if el_name == "userTask" {
                let name = child_el.attribute(&doc, "name")
                    .and_then(|s| Some(s.to_owned()));
                let from_key = child_el.attribute(&doc, "fromKey")
                    .and_then(|s| Some(s.to_owned()));
                let candidate_groups = child_el.attribute(&doc, "candidateGroups")
                    .and_then(|s| Some(s.to_owned()));
                let candidate_users = child_el.attribute(&doc, "candidateUsers")
                    .and_then(|s| Some(s.to_owned()));

                let node = Arc::new(UserTask::new(id.to_owned(), name, from_key, description.clone(), candidate_groups, candidate_users));
                Self::add_node(id, node, pe_elements, element_map)?;
            } else if el_name == "serviceTask" {
                let name = child_el.attribute(&doc, "name")
                    .and_then(|s| Some(s.to_owned()));
                let from_key = child_el.attribute(&doc, "fromKey")
                    .and_then(|s| Some(s.to_owned()));
                let candidate_groups = child_el.attribute(&doc, "candidateGroups")
                    .and_then(|s| Some(s.to_owned()));
                let candidate_users = child_el.attribute(&doc, "candidateUsers")
                    .and_then(|s| Some(s.to_owned()));

                let node = Arc::new(ServiceTask::new(id.to_owned(), name, from_key, description.clone(), candidate_groups, candidate_users));
                Self::add_node(id, node, pe_elements, element_map)?;
            } else if el_name == "exclusiveGateway" {
                let node = Arc::new(ExclusiveGateway::new(id.to_owned(), description.clone()));
                Self::add_node(id, node, pe_elements, element_map)?;
            } else if el_name == "parallelGateway" {
                let node = Arc::new(ParallelGateway::new(id.to_owned(), description.clone()));
                Self::add_node(id, node, pe_elements, element_map)?;
            } else if el_name == "sequenceFlow" {
                let source = child_el.attribute(&doc, "sourceRef")
                    .unwrap_or("").to_owned();
                let target = child_el.attribute(&doc, "targetRef")
                    .unwrap_or("").to_owned();
                let mut condition_express = None;
                let op_el = child_el.find(&doc, "conditionExpression");
                match op_el {
                    None => {}
                    Some(el_condition) => {
                        let condition = el_condition.text_content(&doc).to_owned();
                        condition_express = Some(condition.trim().to_owned());
                    }
                }
                let edge = Arc::new(SequenceFlow::new(id.to_owned(), source, target, condition_express));
                Self::add_edge(id, edge, pe_elements, element_map)?;
            }
        }

        bpmn_def.validate()?;

        Ok(bpmn_def)
    }

    fn add_node(
        id: &str, 
        node: Arc<dyn BpmnNode>, 
        pe_elements: &mut Vec<BpmnElement>, 
        element_map: &mut HashMap<String, BpmnElement>
    ) -> Result<()> {
        pe_elements.push(BpmnElement::Node(node.clone()));
        if element_map.contains_key(id) {
            let app_err = AppError::new(
                ErrorCode::ParseError,
                Some(&format!("Bmpn 中存在重复的 id = {}", id)), concat!(file!(), ":", line!()),
                None
            );
            Err(app_err)?
        } else {
            element_map.insert(id.to_owned(), BpmnElement::Node(node.clone()));
        }

        Ok(())
    }

    fn add_edge(id: &str, edge: Arc<dyn BpmnEdge>, pe_elements: &mut Vec<BpmnElement>,
                element_map: &mut HashMap<String, BpmnElement>) -> Result<()> {
        pe_elements.push(BpmnElement::Edge(edge.clone()));
        if element_map.contains_key(id) {
            let app_err = AppError::new(
                ErrorCode::ParseError,
                Some(&format!("Bmpn 中存在重复的 id = {}", id)),concat!(file!(), ":", line!()),None
            );
            Err(app_err)?
        } else {
            element_map.insert(id.to_owned(), BpmnElement::Edge(edge.clone()));
        }

        Ok(())
    }

    pub fn create_end_event_terminate_node() -> BpmnElement {
        let data: &str = r#"<?xml version="1.0" encoding="utf-8"?><endEvent id="_endEvent_terminate" />"#;

        let doc = Document::parse_str(data).unwrap();
        let end_event = doc.root_element().unwrap();
        let el_id = end_event.attribute(&doc, "id").unwrap();

        let node = Arc::new(EndEvent::new(el_id.to_owned(), None));

        BpmnElement::Node(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::service::engine::DeploymentBuilder;
    use super::*;

    #[test]
    fn test_parse() {
        let builder = DeploymentBuilder::new();
        let deploy_builder = builder.add_file("bpmn/process1.bpmn.xml").unwrap();

        let bpmn_manager = BpmnManager::new();
        let bpmn_xml = String::from_utf8(
            deploy_builder.new_deployment.new_bytearray.bytes.clone()
                                .unwrap_or(Vec::new())).unwrap();
        bpmn_manager.parse(bpmn_xml).unwrap();
    }

    #[test]
    fn test_parse_from_bytes() {
        let builder = DeploymentBuilder::new();
        let deploy_builder = builder.add_file("bpmn/process1.bpmn.xml").unwrap();

        let bpmn_manager = BpmnManager::new();
        bpmn_manager.parse_from_bytes(
            deploy_builder.new_deployment.new_bytearray.bytes.clone().unwrap_or(Vec::new())).unwrap();
    }

    #[test]
    fn test_create_end_event_node() {
        let _rst = BpmnManager::create_end_event_terminate_node();
    }
}