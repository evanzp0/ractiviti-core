
#[allow(non_camel_case_types, unused)]
#[derive(Clone, Debug)]
pub enum TypeWrapper {
    str(String),
    i32(i32),
    f32(f32),
    f64(f64),
    bool(bool),
}

#[allow(unused)]
impl TypeWrapper {
    pub fn as_str(&self) -> String {
        match self {
            TypeWrapper::str(v) => v.to_owned(),
            TypeWrapper::i32(v) => v.to_string(),
            TypeWrapper::f32(v) => v.to_string(),
            TypeWrapper::f64(v) => v.to_string(),
            TypeWrapper::bool(v) => {
                if *v {
                    1.to_string()
                } else {
                    0.to_string()
                }
            },
        }
    }

    pub fn as_i32(&self) -> i32 {
        let mut rst = 0;
        if let TypeWrapper::i32(v) = self {
            rst = *v;
        } else if let TypeWrapper::bool(v) = self {
            if *v == true {
                rst = 1;
            } else {
                rst = 0;
            }
        }

        rst
    }

    pub fn as_f32(&self) -> f32 {
        let mut rst = 0.0;
        if let TypeWrapper::f32(v) = self {
            rst = *v;
        }

        rst
    }

    pub fn as_f64(&self) -> f64 {
        let mut rst = 0_f64;
        if let TypeWrapper::f64(v) = self {
            rst = *v;
        }

        rst
    }

    pub fn as_bool(&self) -> bool {
        let mut rst = false;
        if let TypeWrapper::bool(v) = self {
            rst = *v;
        }

        rst
    }
}
