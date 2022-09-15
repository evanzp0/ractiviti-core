use std::sync::Arc;
use super::{BpmnNode, NodeType};

#[derive(Debug, Default)]
pub struct UserTask {
    pub id: String,
    pub name: Option<String>,
    pub from_key: Option<String>,
    pub description: Option<String>,
    pub candidate_groups: Arc<Vec<String>>,
    pub candidate_users: Arc<Vec<String>>,
}

impl BpmnNode for UserTask {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_node_type(&self) -> NodeType {
        NodeType::UserTask
    }

    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn get_from_key(&self) -> Option<String> {
        self.from_key.clone()
    }

    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

    fn candidate_groups(&self) -> Arc<Vec<String>> {
        self.candidate_groups.clone()
    }

    fn candidate_users(&self) -> Arc<Vec<String>>{
        self.candidate_users.clone()
    }
}

impl UserTask {
    pub fn new(
        id: String, 
        name: Option<String>, 
        from_key: Option<String>, 
        description: Option<String>,
        candidate_groups: Option<String>, 
        candidate_users: Option<String>
    ) -> Self {
        let mut candidate_groups_arr = vec![];
        let mut candidate_users_arr = vec![];

        if let Some(cand) = candidate_groups {
            let cand = cand.to_lowercase();
            let cand_arr = cand.split(',');

            for cand in cand_arr {
                candidate_groups_arr.push(cand.to_owned());
            }
        }

        if let Some(cand) = candidate_users {
            let cand = cand.to_lowercase();
            let cand_arr = cand.split(',');

            for cand in cand_arr {
                candidate_users_arr.push(cand.to_owned());
            }
        }

        Self {
            id,
            name,
            from_key,
            description,
            candidate_groups: Arc::new(candidate_groups_arr),
            candidate_users: Arc::new(candidate_users_arr),
        }
    }
}