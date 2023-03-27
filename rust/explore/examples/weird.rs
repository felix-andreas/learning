fn main() {
    let first = |word: &str| word.chars().next().unwrap().to_string();
    let result = ["Hello", "World"]
        .into_iter()
        // didn't work in rust 2018, because of implicit deref of into_iter of array to slice.iter()
        // .map(|word| first(word)) // <- Why does this work?
        .map(first) // <- But this not?
        .collect::<Vec<String>>();
    println!("{result:?}");
}
