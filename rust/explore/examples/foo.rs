use rand::Rng;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        let number = rand::thread_rng().gen_range(0.0..1.0);
        sleep(Duration::from_millis(100)).await;

        let number2 = rand::thread_rng().gen_range(0.0..1.0);
        println!("{number}{number2}");
        6
    });
    let result = handle.await;
    println!("{:?}", result);
}
