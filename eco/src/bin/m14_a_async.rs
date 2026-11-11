use std::time::Duration;
use tokio::time::{sleep, timeout};

async fn telecharger(nom: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await; // rend la main au runtime pendant l'attente
    format!("{nom} reçu")
}

#[tokio::main]
async fn main() {
    // Séquentiel : 300 ms
    let a = telecharger("a", 100).await;
    let b = telecharger("b", 200).await;
    println!("{a}, {b}");

    // Concurrent dans la même tâche : 200 ms
    let (a, b) = tokio::join!(telecharger("a", 100), telecharger("b", 200));
    println!("{a}, {b}");

    // Tâche indépendante, répartie sur les threads du runtime
    let poignee = tokio::spawn(async { telecharger("c", 50).await });
    println!("{}", poignee.await.expect("la tâche a paniqué"));

    // Délai maximal
    match timeout(Duration::from_millis(80), telecharger("lent", 500)).await {
        Ok(texte) => println!("{texte}"),
        Err(_) => println!("trop lent, abandon"),
    }
}
