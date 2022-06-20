use bytes::Bytes;

use mini_redis::client;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() {
    // Create a new channel with a capacity of at most 32.
    let (tx, mut rx) = mpsc::channel(32);

    // spawn a new task by cloning the sender.
    let tx2 = tx.clone();

    // Eg: move the sender into a task
    // tokio::spawn(async move {
    //     tx.send("sending from first handle").await;
    // });
    // tokio::spawn(async move {
    //     tx2.send("sending from second handle").await;
    // });

    // receiver gets tasks:
    // while let Some(message) = rx.recv().await {
    //     println!("GOT = {:?}", message);
    // }

    // Spawn two tasks
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "hello".to_string(),
        };
        tx.send(cmd).await.unwrap();
    });
    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        };
        tx2.send(cmd).await.unwrap();
    });
    // logging from receiver

    // move ownership of rx into the task manager
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key } => {
                    client.get(&key).await;
                }
                Set { key, val } => {
                    client.set(&key, val).await;
                }
            }
        }
    });
}
