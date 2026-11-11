use std::thread;

fn main() {
    let donnees: Vec<i32> = (1..=3).collect();
    let poignee = thread::spawn(move || { // move : le thread possède donnees
        donnees.iter().sum::<i32>()
    });
    let somme = poignee.join().expect("le thread a paniqué");
    println!("{somme}");

    // Threads à portée limitée : ils peuvent emprunter des variables locales
    let nombres: Vec<u64> = (1..=1_000).collect();
    let (gauche, droite) = nombres.split_at(nombres.len() / 2);
    let total = thread::scope(|s| {
        let h1 = s.spawn(|| gauche.iter().sum::<u64>());
        let h2 = s.spawn(|| droite.iter().sum::<u64>());
        h1.join().unwrap() + h2.join().unwrap()
    });
    println!("{total}"); // 500500
}
