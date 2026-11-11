async fn empreinte(donnees: Vec<u8>) -> Result<u64, tokio::task::JoinError> {
    tokio::task::spawn_blocking(move || {
        // calcul long : hors des threads qui servent les tâches asynchrones
        donnees.iter().fold(0u64, |h, &o| h.wrapping_mul(31).wrapping_add(u64::from(o)))
    })
    .await
}

// --- harnais de vérification, hors cours ---
#[tokio::main]
async fn main() {
    println!("{:?}", empreinte(b"bonjour".to_vec()).await);
}
