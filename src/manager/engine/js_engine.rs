use std::{collections::HashMap};
use boa::{JsValue, Context, JsString};
use crate::{error::{AppError, ErrorCode}, model::WrappedValue};
use color_eyre::Result;
use crate::ArcRw;

pub fn run_script(js_code: String, global_vars: &HashMap<String, JsValue>) -> Result<JsValue>{

    let context = &mut Context::default();
    let g_obj = context.global_object();

    for (k, v) in global_vars.iter() {
        g_obj.set(
            k.to_string(), 
            v, 
            true, 
            context
        )
        .map_err(|e| {
            let s = format!("Uncaught {}", e.display());
            AppError::new(ErrorCode::InternalError, Some(&s), concat!(file!(), ":", line!()), None)
        })?;
    }

    let rst =match context.eval(js_code) {
        Ok(res) => Ok(res),
        Err(e) => {
            // Pretty print the error
            let s = format!("Uncaught {}", e.display());
            Err(AppError::new(ErrorCode::InternalError, Some(&s), concat!(file!(), ":", line!()), None))?
        }
    };

    rst
}

pub fn convert_map(type_wrap_map: ArcRw<HashMap<String, WrappedValue>>) -> HashMap<String, JsValue> {
    let mut rst: HashMap<String, JsValue> = HashMap::new();

    type_wrap_map.read().unwrap().iter().for_each(|(key, value)| {
        let v  = match value {
            WrappedValue::Str(v) => JsValue::String(JsString::new(v)),
            WrappedValue::Int(v) => JsValue::Integer(*v),
            WrappedValue::Double(v) => JsValue::Rational(*v),
            WrappedValue::Bool(v) => JsValue::Boolean(*v),
        };
        rst.insert(key.clone(), v);
    });

    rst
}