use tokio::signal;

#[tokio::main]
async fn main() {
    loop {
        _ = signal::ctrl_c().await;
        println!("ctrl-c received!");
    }
}

fn foo() -> Result<u32, ()> {
    Err(())
}
