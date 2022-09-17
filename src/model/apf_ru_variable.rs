use std::collections::HashMap;
use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

use super::{WrappedValue, WrappedValueType};

#[derive(Debug, Serialize, PartialEq, Default, Clone)]
#[derive(PostgresMapper)]
#[pg_mapper(table="apf_ru_variable")]
pub struct ApfRuVariable {
    pub id: String,
    pub rev: i32,
    pub var_type: WrappedValueType,
    pub name: String,
    pub value: String,
    pub proc_inst_id: String,
    pub execution_id: Option<String>,
    pub task_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct ApfRuVariableDto {
    pub var_type: WrappedValueType,
    pub name: String,
    pub value: String,
    pub proc_inst_id: String,
    pub execution_id: Option<String>,
    pub task_id: Option<String>,
}

impl ApfRuVariable {
    pub fn get_value_as_i32(&self) -> i32 {
        self.value.parse::<i32>().unwrap_or(0_i32) as i32
    }

    pub fn get_value_as_f64(&self) -> f64 {
        self.value.parse::<f64>().unwrap_or(0_f64) as f64
    }

    pub fn get_value_as_bool(&self) -> bool {
        if self.value.trim() == "0" {
            false
        } else {
            true
        }
    }

    pub fn convert_variables_to_map(variables: &Vec<ApfRuVariable>) -> HashMap<String, WrappedValue> {
        let mut rst_map: HashMap<String, WrappedValue> = HashMap::new();
        
        for ru_var in variables {
            rst_map.insert(ru_var.name.clone(), WrappedValue::from(ru_var.clone()));
        }

        rst_map
    }
}

impl From<ApfRuVariable> for WrappedValue {
    fn from(v: ApfRuVariable) -> Self {
        match v.var_type {
            WrappedValueType::INT => WrappedValue::Int(v.get_value_as_i32()),
            WrappedValueType::DOUBLE => WrappedValue::Double(v.get_value_as_f64()),
            WrappedValueType::STRING => WrappedValue::Str(v.value),
            WrappedValueType::BOOL => WrappedValue::Bool(v.get_value_as_bool()),
        }
    }
}
