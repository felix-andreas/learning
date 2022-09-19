#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn threads() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1, 2, 3];
    println!("{data:?}");

    let handle = thread::spawn(move || {
        for i in 0..5 {
            println!("thread {i}");
            thread::sleep(Duration::from_millis(200));
        }
        data[0]
    });

    println!("Main Thread");
    thread::sleep(Duration::from_secs(1));
    let result = handle.join().unwrap();
    println!("Thread returned: {result}");
    Ok(())
}

fn channels() -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = ["hi", "there"].map(|s| s.to_string());
        for val in vals {
            thread::sleep(Duration::from_millis(5));
            tx.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let vals = ["hi", "there"].map(|s| s.to_string());
        for val in vals {
            thread::sleep(Duration::from_millis(5));
            tx1.send(val).unwrap();
        }
    });

    let result = rx.recv().unwrap();
    println!("Got: {result}");

    for result in rx {
        println!("Got: {result}");
    }
    Ok(())
}

fn shared_state() {
    let counter = Arc::new(Mutex::new(0));
    {
        let mut num = counter.lock().unwrap();
        *num = 10;
    }

    (0..10)
        .into_iter()
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        })
        .for_each(|handle| handle.join().unwrap());

    println!("counter = {}", *counter.lock().unwrap());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // threads()?;
    // channels()?;
    shared_state();
    Ok(())
}
