use crate::core::core::Core;

pub struct Module {
    pub core: Core
}

impl Module {
    pub fn new() -> Self {
        let mut core: Core = Core::new();
        Self {
            core
        }
    }

}
