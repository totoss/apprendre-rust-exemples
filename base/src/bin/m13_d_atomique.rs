use std::sync::atomic::{AtomicU64, Ordering};

static REQUETES: AtomicU64 = AtomicU64::new(0);

fn compter() -> u64 {
    REQUETES.fetch_add(1, Ordering::Relaxed) + 1
}

// --- harnais de vérification, hors cours ---
fn main() {
    compter();
    println!("{}", compter());
}
