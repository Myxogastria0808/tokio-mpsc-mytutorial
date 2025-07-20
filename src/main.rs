use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Create a channel with a buffer size of 2
    // Sender (producer) ... tx
    // Receiver (consumer) ... rx
    let (tx, mut rx) = mpsc::channel(2);
    // Spawn a task to send messages multiple times
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    tokio::spawn(async move {
        tx.send("Hello from tx").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("Hello from tx2").await.unwrap();
    });

    tokio::spawn(async move {
        tx3.send("Hello from tx3").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("Received: {message}");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
