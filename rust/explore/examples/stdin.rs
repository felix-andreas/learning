fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let result = stdin
        .split('\n')
        .enumerate()
        .map(|(i, line)| format!("{i:>3}| {line}\n"))
        .collect::<String>();
    println!("{result}");
}
