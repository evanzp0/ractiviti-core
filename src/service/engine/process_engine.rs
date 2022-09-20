use std::sync::Arc;
use super::RuntimeService;
use super::RepositoryService;
use super::HistoryService;
use super::TaskService;

#[derive(Debug)]
pub struct ProcessEngine {
    name: String,
    repository_service: Arc<RepositoryService>,
    runtime_service: Arc<RuntimeService>,
    history_service: Arc<HistoryService>,
    task_service: Arc<TaskService>,
}

#[allow(unused)]
impl ProcessEngine {

    pub const DEFAULT_ENGINE: &'static str = "default";

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            repository_service: Arc::new(RepositoryService::new()),
            runtime_service: Arc::new(RuntimeService::new()),
            history_service: Arc::new(HistoryService::new()),
            task_service: Arc::new(TaskService::new()),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_repository_service(&self) -> Arc<RepositoryService> {
        self.repository_service.clone()
    }

    pub fn get_runtime_service(&self) -> Arc<RuntimeService> {
        self.runtime_service.clone()
    }

    pub fn get_history_service(&self) -> Arc<HistoryService> {
        self.history_service.clone()
    }

    pub fn get_task_service(&self) -> Arc<TaskService> {
        self.task_service.clone()
    }
}