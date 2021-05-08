use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match *self {
            Object::Null => false,
            Object::Boolean(value) => value,
            _ => true,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Object::Null => write!(f, "nil"),
            Object::Boolean(v) => write!(f, "{}", v.to_string()),
            Object::Number(v) => {
                let mut text = v.to_string();
                if let Some(value) = text.strip_prefix(".0") {
                    text = value.to_string();
                }
                write!(f, "{}", text)
            }
            Object::String(v) => write!(f, "{}", v.to_string()),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // compare same types
            (Object::Null, Object::Null) => true,
            (Object::Number(value1), Object::Number(value2)) => value1 == value2,
            (Object::String(value1), Object::String(value2)) => value1 == value2,
            (Object::Boolean(value1), Object::Boolean(value2)) => value1 == value2,

            // any other type combinations including null case from book
            _ => false,
        }
    }
}
