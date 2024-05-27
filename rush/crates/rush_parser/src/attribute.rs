#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    key: String,
    value: Option<String>,
}

impl Attribute {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

