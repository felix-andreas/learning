use std::mem;

fn main() {
    let logical = true;
    if logical {
        let mut mutable = 10;
        mutable += 1;
        println!("the mutable is {}", mutable);
        println!("the integer division is {}", mutable / 2);
    }

    let tuple = ("hihi", 12e14);
    let (x, y) = tuple;
    println!("unpacked x and y to {} and {:e}", x, y);

    let xs: [i32; 6] = [0, 2, 4, 6, 8, 10];

    println!("the second to last entry is: {}", xs[xs.len() - 1]);
    println!("size of xs in bytes is: {}", mem::size_of_val(&xs));
}
