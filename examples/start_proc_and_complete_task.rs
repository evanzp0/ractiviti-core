#![allow(unused_imports)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use log4rs_macros::debug;
use ractiviti_core::common::db;
use ractiviti_core::service::engine::ProcessEngine;
use ractiviti_core::service::engine::query::TaskQuery;
use ractiviti_core::model::WrappedValue;

#[tokio::test]
async fn test_deploy() {
    log4rs_macros::prepare_log();

    let process_engine = ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE);
    let repository_service = process_engine.get_repository_service();
    let deployment = repository_service.create_deployment_builder()
        .add_file("bpmn/process_2.bpmn.xml").unwrap()
        .name("test_deploy_2")
        .key("delopy_biz_key_2")
        .deployer_id("test_user_1")
        .company_id("test_comp_1")
        .deply()
        .await
        .unwrap();

    debug!(deployment)
}

#[tokio::test]
async fn test_start_process() {
    log4rs_macros::prepare_log();

    let process_engine = ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE);
    let runtime_service = process_engine.get_runtime_service();
    let _proc_inst = runtime_service.start_process_instance_by_key(
        "bpmn_process_2",
        Some("process_biz_key_2".to_owned()),
        HashMap::new(),
        None,
        None
    )
    .await
    .unwrap();

}

#[tokio::test]
async fn test_complete() {
    log4rs_macros::prepare_log();

    let process_engine = ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE);
    let task_service = process_engine.get_task_service();
    let mut conn = db::get_connect().await.unwrap();
    let tran = conn.transaction().await.unwrap();
    let task = TaskQuery::new(&tran)
        .process_definition_key("bpmn_process_2")
        .business_key("process_biz_key_2")
        .candidate_user(Some("user_1".to_owned()))
        .fetch_one()
        .await.unwrap();
    tran.commit().await.unwrap();

    let mut variables = HashMap::new();
    variables.insert("approval_pass".to_owned(), WrappedValue::Bool(true));

    task_service.complete(&task.id, variables, Some("user_1".to_owned()), None).await.unwrap();
}

#[tokio::main]
async fn main() {

}