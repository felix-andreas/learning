fn main() {
    let string = "Hello World!".to_owned();
    let length = calculate_length(&string);
    println!("'{string}' has length {length}.");
}

fn calculate_length(string: &str) -> usize {
    string.len()
}
