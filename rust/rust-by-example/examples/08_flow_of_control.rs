fn main() {
    let result = 'outer: loop {
        println!("Enter outer loop");
        loop {
            println!("Enter inner loop");
            break 'outer 25;
        }
        // This point will never be reached!
    };

    println!("The result is {}", result);
}
