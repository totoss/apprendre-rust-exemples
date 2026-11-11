// src/main.rs
use taches::Tache;

fn main() {
    let mut t = Tache::new(1, "apprendre les modules");
    t.terminer();
    println!("{t:?} {}", t.est_terminee());
}
