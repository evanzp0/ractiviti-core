use std::fmt::{Display, Formatter};
use rstring_builder::{StringBuilder, Vcharsable};

#[allow(non_camel_case_types, unused)]
pub enum SqlFragment {
    SELECT,
    COUNT(String),
    WHERE,
    AND(String),
    FIELD(String),
    FROM(String),
    ORDER_BY(String),
    JION(String),
    LEFT_JION(String),
    DISTINCT,
}

impl SqlFragment {
    pub const BLANK: &'static str = "";
}
impl Display for SqlFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rst = match self {
            Self::SELECT => "select ".to_owned(),
            Self::DISTINCT => " distinct ".to_owned(),
            Self::COUNT(v) => format!(" count({}) ", v),
            Self::WHERE => " where 1 = 1 ".to_owned(),
            Self::JION(s) => {
                let mut sb = StringBuilder::new();
                sb.append(" join ")
                    .append(s.to_owned())
                    .append(Self::BLANK)
                    .string()
            },
            Self::LEFT_JION(s) => {
                let mut sb = StringBuilder::new();
                sb.append(" left join ")
                    .append(s.to_owned())
                    .append(Self::BLANK)
                    .string()
            },
            Self::AND(s) => {
                let mut sb = StringBuilder::new();
                    sb.append(" and ")
                    .append(s.to_owned())
                    .append(Self::BLANK)
                    .string()
            },
            Self::FIELD(s) => {
                let mut sb = StringBuilder::new();
                sb.append(Self::BLANK)
                    .append(s.to_owned())
                    .append(Self::BLANK)
                    .string()
            },
            Self::FROM(s) => {
                let mut sb = StringBuilder::new();
                sb.append(" from ")
                    .append(s.to_owned())
                    .append(Self::BLANK)
                    .string()
            },
            Self::ORDER_BY(s) => {
                let mut sb = StringBuilder::new();
                sb.append(" order by ")
                    .append(s.to_owned())
                    .append(Self::BLANK)
                    .string()
            },
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