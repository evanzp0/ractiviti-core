use std::fmt::{Display, Formatter};

use crate::common::{Vcharsable, StringBuilder};

#[allow(non_camel_case_types, unused)]
pub enum SqlFragment {
    SELECT,
    COUNT(String),
    WHERE,
    AND(String),
    FIELD(String),
    FROM(String),
    ORDER_BY(String),
    JOIN(String),
    LEFT_JOIN(String),
    DISTINCT,
}

impl SqlFragment {
    pub const BLANK: &'static str = " ";
}

impl Display for SqlFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sb = StringBuilder::new();
        let rst = match self {
            Self::SELECT => sb.append("SELECT").string(),
            Self::DISTINCT => sb.append(Self::BLANK).append("DISTINCT").string(),
            Self::COUNT(v) => sb.append(Self::BLANK).append(format!("COUNT({})", v)).string(),
            Self::WHERE => sb.append(Self::BLANK).append("WHERE 1 = 1").string(),
            Self::JOIN(s) => sb.append(Self::BLANK).append("JOIN").append(Self::BLANK).append(s.to_owned()).string(),
            Self::LEFT_JOIN(s) => sb.append(Self::BLANK).append("LEFT JOIN").append(Self::BLANK).append(s.to_owned()).string(),
            Self::AND(s) => sb.append(Self::BLANK).append("AND").append(Self::BLANK).append(s.to_owned()).string(),
            Self::FIELD(s) => sb.append(Self::BLANK).append(s.to_owned()).string(),
            Self::FROM(s) => sb.append(Self::BLANK).append("from").append(Self::BLANK).append(s.to_owned()).string(),
            Self::ORDER_BY(s) => sb.append(Self::BLANK).append("order by").append(Self::BLANK).append(s.to_owned()).string(),
        };

        write!(f, "{}", rst)
    }
}

impl Vcharsable for SqlFragment {
    fn vechars(&self) -> Vec<char> {
        let r = self.to_string().chars().collect();
        r
    }
}