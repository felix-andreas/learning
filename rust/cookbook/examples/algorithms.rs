use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Integer between 1 and 10: {}", rng.gen_range(1..=10));
}
