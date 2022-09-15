use std::sync::Arc;
use super::BpmnProcess;
use color_eyre::Result;
use crate::error::{AppError, ErrorCode};
use crate::manager::engine::{BpmnEdge, BpmnElement, BpmnNode, NodeType};

#[derive(Debug, Default)]
pub struct BpmnDefinitions {
    pub xml: String,
    pub process: BpmnProcess
}

impl BpmnDefinitions {
    pub fn new(xml: String, process: BpmnProcess) -> Self {
        Self {
            xml,
            process,
        }
    }

    pub fn validate(&self) -> Result<()> {
        let bpmn_proc = &self.process;
        let elements = &bpmn_proc.elements;

        for el in elements {
            match el {
                BpmnElement::Node(node) => {
                    let in_flows_len = node.in_flows(bpmn_proc).len();
                    let out_flows_len= node.out_flows(bpmn_proc).len();

                    match node.get_node_type() {
                        NodeType::StartEvent => {
                            Self::allow_zero_inflow(node, in_flows_len)?;
                            Self::allow_one_outflow(node, out_flows_len)?;
                        }
                        NodeType::EndEvent => {
                            Self::allow_at_least_one_inflow(node, in_flows_len)?;
                            Self::allow_zero_outflow(node, out_flows_len)?;
                        }
                        NodeType::UserTask => {
                            Self::allow_at_least_one_inflow(node, in_flows_len)?;
                            Self::allow_one_outflow(node, out_flows_len)?;
                        }
                        NodeType::ServiceTask => {
                            Self::allow_at_least_one_inflow(node, in_flows_len)?;
                            Self::allow_one_outflow(node, out_flows_len)?;
                        }
                        NodeType::ExclusiveGateway => {
                            Self::allow_at_least_one_inflow(node, in_flows_len)?;
                            Self::allow_at_least_one_outflow(node, out_flows_len)?;
                        }
                        NodeType::ParallelGateway => {
                            Self::allow_at_least_one_inflow(node, in_flows_len)?;
                            Self::allow_at_least_one_outflow(node, out_flows_len)?;
                        }
                    }
                },
                BpmnElement::Edge(edge) => {
                    Self::allow_source_and_target(edge, bpmn_proc)?;
                }
            }
        }

        Ok(())
    }

    // validate rules for node ---------------------------------------------------------

    fn allow_zero_inflow(node: &Arc<dyn BpmnNode>, in_flows_len: usize) -> Result<()> {
        if in_flows_len != 0_usize {
            let msg = format!("{:?}({}) 不能有输入边 (len: {})",
                              node.get_node_type(), node.get_id(), in_flows_len);

            Err(AppError::new(
                ErrorCode::ParseError,
                Some(&msg),
                concat!(file!(), ":", line!()),
                None
            ))?;
        }

        Ok(())
    }

    fn allow_zero_outflow(node: &Arc<dyn BpmnNode>, out_flows_len: usize) -> Result<()> {
        if out_flows_len != 0_usize {
            let msg = format!("{:?}({}) 不能有输出边 (len: {})",
                              node.get_node_type(), node.get_id(), out_flows_len);

            Err(AppError::new(
                ErrorCode::ParseError,
                Some(&msg),
                concat!(file!(), ":", line!()),
                None
            ))?;
        }

        Ok(())
    }

    fn allow_one_outflow(node: &Arc<dyn BpmnNode>, out_flows_len: usize) -> Result<()> {
        if out_flows_len != 1_usize {
            let msg = format!("{:?}({}) 有且只能有1条输出边 (len: {})",
                              node.get_node_type(), node.get_id(), out_flows_len);
            Err(AppError::new(
                ErrorCode::ParseError,
                Some(&msg),
                concat!(file!(), ":", line!()),
                None
            ))?;
        }

        Ok(())
    }

    fn allow_at_least_one_inflow(node: &Arc<dyn BpmnNode>, in_flows_len: usize) -> Result<()> {
        if in_flows_len == 0_usize {
            let msg = format!("{:?}({}) 至少要有1条输入边 (len: {})",
                              node.get_node_type(), node.get_id(), in_flows_len);

            Err(AppError::new(
                ErrorCode::ParseError,
                Some(&msg),
                concat!(file!(), ":", line!()),
                None
            ))?;
        }

        Ok(())
    }

    fn allow_at_least_one_outflow(node: &Arc<dyn BpmnNode>, out_flows_len: usize) -> Result<()> {
        if out_flows_len == 0_usize {
            let msg = format!("{:?}({}) 至少要有1条输出边 (len: {})",
                              node.get_node_type(), node.get_id(), out_flows_len);

            Err(AppError::new(
                ErrorCode::ParseError,
                Some(&msg),
                concat!(file!(), ":", line!()),
                None
            ))?;
        }

        Ok(())
    }

    // validate rules for edge ---------------------------------------------------------
    fn allow_source_and_target(edge: &Arc<dyn BpmnEdge>, bpmn_proc: &BpmnProcess) -> Result<()> {
        let from_node = edge.from_node(bpmn_proc);
        let to_node = edge.to_node(bpmn_proc);

        match from_node {
            None => {
                let msg = format!("SequenceFlow ({}) 必须要有输入节点", edge.get_id());

                Err(AppError::new(
                    ErrorCode::ParseError,
                    Some(&msg),
                    concat!(file!(), ":", line!()),
                    None
                ))?
            },
            Some(_) => {}
        }

        match to_node {
            None => {
                let msg = format!("SequenceFlow ({}) 必须要有输出节点", edge.get_id());

                Err(AppError::new(
                    ErrorCode::ParseError,
                    Some(&msg),
                    concat!(file!(), ":", line!()),
                    None
                ))?
            },
            Some(_) => {}
        }

        Ok(())
    }

}