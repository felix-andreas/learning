use serde::Serialize;

#[derive(Serialize)]
enum Collection {
    Foo,
    Bar,
}

fn main() {
    let collection = Collection::Foo;
    println!("{}", serde_json::to_string(&collection).unwrap());
}
