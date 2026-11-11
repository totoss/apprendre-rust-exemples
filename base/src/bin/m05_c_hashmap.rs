use std::collections::HashMap;

fn main() {
    let texte = "le chat et le chien et le rat";
    let mut frequences: HashMap<&str, u32> = HashMap::new();
    for mot in texte.split_whitespace() {
        *frequences.entry(mot).or_insert(0) += 1; // insère 0 si absent, puis incrémente
    }

    if let Some(n) = frequences.get("le") {
        println!("le : {n}");
    }
    let absent = frequences.get("loup").copied().unwrap_or(0);

    let mut tri: Vec<(&str, u32)> = frequences.into_iter().collect();
    tri.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0))); // fréquence décroissante, puis alphabétique
    println!("{tri:?} {absent}");
}
