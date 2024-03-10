use crate::module::module::Module;
use crate::tracer::tracer::Tracer;

pub struct Rush {
    working: bool,
    top: Module,
    tracer: Option<Tracer>,
}

impl Rush {
    pub fn new(top: Module) -> Self {
        Self {
            working: false,
            top,
            tracer: None,
        }
    }

    pub fn run(&self) -> u64 {
        let mut steps: u64 = 0;
        loop {
            if !self.top.core.step().to_bool() {
                break
            }
            steps += 1;
        }
        steps
    }
}