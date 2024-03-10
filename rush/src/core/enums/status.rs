pub enum Status {
    CoreExecuted,
    NotExecuted
}

impl Status {
    pub fn to_bool(&self) -> bool {
        match self {
            Self::CoreExecuted => true,
            Self::NotExecuted => false
        }
    }
}