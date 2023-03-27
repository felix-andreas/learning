#[derive(Debug)]
struct Foo(i32);

fn main() {
    let x = Foo(5);
    let y = &x;
    bar(*y);
}

fn bar(foo @ Foo(value): Foo) {
    println!("{foo:?} holds the value '{value}'");
}
