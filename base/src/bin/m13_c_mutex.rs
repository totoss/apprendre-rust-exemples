use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let compteurs: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut poignees = Vec::new();

    for mot in ["rust", "go", "rust", "zig", "rust"] {
        let compteurs = Arc::clone(&compteurs);
        poignees.push(thread::spawn(move || {
            let mut table = compteurs.lock().unwrap(); // attend le verrou
            *table.entry(mot.to_string()).or_insert(0) += 1;
        })); // le verrou est rendu ici, à la libération de table
    }
    for p in poignees {
        p.join().unwrap();
    }
    println!("{:?}", compteurs.lock().unwrap());
}
