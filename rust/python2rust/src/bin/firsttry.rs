#![feature(rustc_private)]
#![deny(rustc::internal)]
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_interface;
extern crate rustc_lint;

struct MyCallbacks;
impl rustc_driver::Callbacks for MyCallbacks {}

fn main() -> Result<(), rustc_errors::ErrorReported> {
    let args: Vec<_> = std::env::args().collect();
    let mut my_callback = MyCallbacks;
    rustc_driver::RunCompiler::new(&args, &mut my_callback).run()
}
