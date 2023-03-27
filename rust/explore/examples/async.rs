/// async basics

use std::{future::Future, task::Poll};

async fn greet() -> i32 {
    println!("Hello world!");
    return 5;
}

fn main() {
    let mut future = greet();
    let result = loop {
        match future.poll() {
            Poll::Pending => {}
            Poll::Ready(value) => break value,
        }
    };
    println!("Future returned '{result}'");
}
