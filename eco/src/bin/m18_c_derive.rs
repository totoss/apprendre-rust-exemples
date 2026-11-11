// --- harnais de vérification, hors cours : utilise la macro derive du module 18 ---
use decrire_derive::Decrire;

trait Decrire {
    fn decrire() -> &'static str;
}

#[derive(Decrire)]
struct Capteur;

fn main() {
    println!("{}", Capteur::decrire());
}
