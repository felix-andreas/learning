// based on https://www.youtube.com/watch?v=FNcXf-4CLH0

use futures::executor::block_on;
use std::sync::{Arc, Mutex};

struct State {
    count: i32,
}

async fn task(state: &Arc<Mutex<State>>, number: i32) -> i32 {
    if let Ok(mut state) = state.lock() {
        state.count += number;
        println!("task: add {} -> state.count {}", number, state.count);
    }
    number
}

async fn async_main() {
    let state = State { count: 0 };
    let data = Arc::new(Mutex::new(state));

    println!("Before first state");
    task(&data, 1).await;
    println!("Before second state");
    task(&data, 2).await;
    let (number_3, number_4) = futures::join!(task(&data, 3), task(&data, 4));
    println!("return values {}, {}", number_3, number_4);
}

fn main() {
    block_on(async_main())
}
