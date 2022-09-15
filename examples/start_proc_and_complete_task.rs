#![allow(unused_imports)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use log4rs::debug;
use sqlx::Acquire;
use approval_flow_core::boot::db;
use approval_flow_core::manager::engine::{ProcessEngine, TypeWrapper};
use approval_flow_core::manager::engine::query::TaskQuery;

#[tokio::test]
async fn test_deploy() {
    log4rs::prepare_log();

    let process_engine = ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE);
    let repository_service = process_engine.get_repository_service();
    let deployment = repository_service.create_deployment_builder()
        .add_file("bpmn/process_2.bpmn.xml").unwrap()
        .name("test_deploy_2").unwrap()
        .key("delopy_biz_key_2").unwrap()
        .deply().await
        .unwrap();

    debug!(deployment)
}

#[tokio::test]
async fn test_start_process() {
    log4rs::prepare_log();

    let process_engine = ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE);
    let runtime_service = process_engine.get_runtime_service();
    let _proc_inst = runtime_service.start_process_instance_by_key(
        "bpmn_process_2",
        Some("process_biz_key_2".to_owned()),
        None,
        None,
        None
    )
    .await
    .unwrap();

}

#[tokio::test]
async fn test_complete() {
    log4rs::prepare_log();

    let process_engine = ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE);
    let task_service = process_engine.get_task_service();
    let mut conn = db::get_connect().await.unwrap();
    let mut tran = conn.begin().await.unwrap();
    let task = TaskQuery::new()
        .process_definition_key("bpmn_process_2")
        .business_key("process_biz_key_2")
        .candidate_user(Some("user_1".to_owned()))
        .fetch_one(&mut tran)
        .await.unwrap();
    tran.commit().await.unwrap();

    let mut variables = HashMap::new();
    variables.insert("approval_pass".to_owned(), TypeWrapper::bool(true));
    let variables = Arc::new(RwLock::new(variables));

    task_service.complete(&task.id, Some(variables), Some("user_1".to_owned()), None).await.unwrap();
}

#[tokio::main]
async fn main() {

}