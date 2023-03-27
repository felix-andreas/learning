use std::process::Termination;

use rand::Rng;

struct Foo(u8);

impl Termination for Foo {
    fn report(self) -> std::process::ExitCode {
        self.0.into()
    }
}

fn main() -> Result<Foo, i32> {
    if rand::thread_rng().gen_bool(0.5) {
        Ok(Foo(42))
    } else {
        Err(42)
    }
}
