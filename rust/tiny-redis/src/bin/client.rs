use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        responder: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        responder: Responder<()>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<Command>(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, responder } => {
                    let result = client.get(&key).await;
                    let _ = responder.send(result);
                }
                Set {
                    key,
                    value,
                    responder,
                } => {
                    let result = client.set(&key, value).await;
                    let _ = responder.send(result);
                }
            };
        }
    });

    tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(Command::Get {
            key: "foo".to_string(),
            responder: resp_tx,
        })
        .await
        .unwrap();
        let result = resp_rx.await;
        println!("GET: {result:?}");
    });

    tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "foo".to_string(),
            value: "bar".to_string().into(),
            responder: resp_tx,
        })
        .await
        .unwrap();
        let result = resp_rx.await;
        println!("SET: {result:?}");
    });

    manager.await.unwrap();
    Ok(())
}
