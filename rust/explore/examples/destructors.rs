#[derive(Debug)]
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

// https://doc.rust-lang.org/reference/destructors.html#extending-based-on-expressions
fn main() {
    PrintOnDrop("first");
    let x = Some {
        0: &PrintOnDrop("x"),
    };
    println!("{x:?}");
    let _y = Some(&PrintOnDrop("y"));
    // println!("{y:?}"); // not allowed
    PrintOnDrop("last");
}
