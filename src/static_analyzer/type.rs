#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    String,
    Array(Box<Type>),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Type::*;

        match self {
            Integer => write!(f, "Integer"),
            Float => write!(f, "Float"),
            Boolean => write!(f, "Boolean"),
            String => write!(f, "String"),
            Array(t) => write!(f, "{}[]", t),
            Void => write!(f, "Void"),
        }
    }
}
