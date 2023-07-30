#[tokio::main]
async fn main() {
    let mut h1 = tokio::spawn(task1());
    let mut h2 = tokio::spawn(task2());
    let result = tokio::select! {
        x = (&mut h1) => x,
        x = (&mut h2) => x,
    };
    println!("{result:?}");
}

async fn task1() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let result = tokio::spawn(async {
        panic!();
    })
    .await;
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    return result.unwrap_or(1);
}

async fn task2() -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(3000)).await;
    return 2;
}
