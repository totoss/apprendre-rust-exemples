fn plus_longue<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let titre = String::from("ownership");
    let resultat;
    {
        let court = String::from("move");
        resultat = plus_longue(&titre, &court);
        println!("{resultat}"); // accepté : court vit encore
    }
    // println!("{resultat}"); // erreur E0597 : `court` does not live long enough
}
