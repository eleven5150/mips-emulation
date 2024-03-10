use crate::core::enums::status::Status;

pub struct Core {

}

impl Core {
    pub fn new() -> Self {
        Self { }
    }

    pub fn step(&self) -> Status {
        println!("step");
        Status::CoreExecuted
    }
}