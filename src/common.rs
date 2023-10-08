#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    String,
    Empty,
    Never,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Integer => write!(f, "integer"),
            Type::Float => write!(f, "float"),
            Type::Boolean => write!(f, "boolean"),
            Type::String => write!(f, "string"),
            Type::Empty => write!(f, "empty"),
            Type::Never => write!(f, "never"),
        }
    }
}
