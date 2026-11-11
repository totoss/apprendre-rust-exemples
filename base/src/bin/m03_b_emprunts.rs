fn longueur(s: &str) -> usize { s.len() }     // emprunt partagé : lecture seule
fn crier(s: &mut String) { s.push('!'); }     // emprunt exclusif : modification
fn consommer(s: String) -> usize { s.len() }  // prise de possession

fn main() {
    let mut texte = String::from("salut");
    let n = longueur(&texte); // &String est accepté là où &str est attendu
    crier(&mut texte);
    println!("{texte} {n}");
    let m = consommer(texte);
    // texte n'existe plus ici
    println!("{m}");
}
