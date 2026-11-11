use tokio::sync::mpsc;
use tokio::time::{Duration, interval, sleep};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(32); // canal borné : contre-pression

    tokio::spawn(async move {
        for i in 0..3 {
            tx.send(format!("événement {i}")).await.unwrap();
            sleep(Duration::from_millis(150)).await;
        }
    });

    let mut battement = interval(Duration::from_millis(100));
    loop {
        tokio::select! {
            message = rx.recv() => match message {
                Some(m) => println!("{m}"),
                None => break, // tous les émetteurs ont disparu
            },
            _ = battement.tick() => println!("tic"),
            _ = tokio::signal::ctrl_c() => {
                println!("arrêt demandé");
                break;
            }
        }
    }
}
