/// async functions are lazy in rust

use std::time::Instant;

use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    // Rust's futures are lazy
    let time = Instant::now();
    let future_a = sleep_n_greet(1);
    let future_b = sleep_n_greet(2);

    future_a.await;
    future_b.await;

    println!("total time {:.2} seconds", time.elapsed().as_secs_f32());

    let time = Instant::now();

    // Use tokio::join to run concurrently
    let future_a = sleep_n_greet(1);
    let future_b = sleep_n_greet(2);

    tokio::join!(future_a, future_b);
    println!("total time {:.2} seconds", time.elapsed().as_secs_f32());
}

async fn sleep_n_greet(seconds: u64) {
    time::sleep(Duration::from_secs(seconds)).await;
    println!("hello after {} seconds", seconds);
}
