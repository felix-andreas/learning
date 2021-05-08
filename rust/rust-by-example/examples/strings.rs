// see https://www.youtube.com/watch?v=ClPrjjHmo2Y

#[allow(unused_variables)]
fn main() {
    let string_slice: &str = "Hällo";
    let string: String = String::from("Partner");
    println!("{} {}!", string_slice, string);

    let string_from_str = string_slice.to_string();
    let str_from_string = &string;

    // combine strings
    let combine = ["first", " ", "second"].concat();
    println!("{}", combine);

    let combine = format!("{} {}", "first", "second");
    println!("{}", combine);

    let combine = string + string_slice;
    println!("{}", combine);

    let mut mut_string = String::new();
    mut_string.push_str("hihi");
    mut_string.push_str(string_slice);
    println!("{}", mut_string);
}
