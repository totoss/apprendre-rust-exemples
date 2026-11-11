// --- harnais de vérification, hors cours ---
// Binaire minimal pour que tests/cli.rs ait quelque chose à lancer :
// il affiche la moyenne des nombres passés en arguments.
fn main() {
    let valeurs: Vec<f64> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    match notes::moyenne(&valeurs) {
        Some(m) => println!("{m}"),
        None => std::process::exit(1),
    }
}
