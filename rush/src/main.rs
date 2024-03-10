use crate::module::module::Module;
use crate::rush::Rush;

mod rush;
mod module;
mod tracer;
mod core;

fn main() {
    let mut top: Module = Module::new();
    let mut rush: Rush = Rush::new(top);
    rush.run();
}
