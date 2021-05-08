#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.123_456_789_f64;
    println!("decimal: {}, u8-cast: {}", decimal, decimal as u8);
    println!("1000 (0b{0:b}) as u8: {1} (0b{1:b})", 1000, 1000 as u8);
    println!("-1 i8 as as u8: {}", -1i8 as u8);
    println!("0b00101010: {}", 0b_0010_1010);
    println!("0b_1111_1111 as i8: {}", 0b_1111_1111_u8 as i8);
    println!("128 as a i8 is : {}", 128 as i8);
    println!("300.0 is {}", 300.0_f32 as u8);

    let mut vec = Vec::new();
    vec.push(123i8);
    vec.push(255u8 as i8);
    println!("{:?}", vec);

    type Nanosecond = u64;

    let nanosecond: Nanosecond = 5;
    println!("nanosecond {}", nanosecond);
}
