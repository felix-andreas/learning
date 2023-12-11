static mut VALUE: i32 = 0;
fn main() {
    let handle = std::thread::spawn(|| {
        for _ in 0..50_000 {
            unsafe {
                VALUE += 1;
            }
        }
    });
    for _ in 0..100_000 {
        unsafe {
            VALUE += 1;
        }
    }
    handle.join().unwrap();
    unsafe {
        println!("{VALUE}");
    }
}
