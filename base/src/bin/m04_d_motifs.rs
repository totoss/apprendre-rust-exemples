fn decrire(n: u32) -> &'static str {
    match n {
        0 => "zéro",
        1 | 2 => "petit",      // alternatives
        3..=9 => "moyen",      // intervalle inclusif
        _ => "grand",          // tout le reste
    }
}

fn main() {
    let age = 42;
    let tranche = match age {
        n @ 0..=17 => format!("mineur ({n})"), // @ lie la valeur testée
        n => format!("majeur ({n})"),
    };
    let est_voyelle = matches!('e', 'a' | 'e' | 'i' | 'o' | 'u' | 'y');
    println!("{tranche} {est_voyelle} {}", decrire(5));

    let mut pile = vec![1, 2, 3];
    while let Some(sommet) = pile.pop() { // boucle tant que le motif correspond
        println!("{sommet}");
    }
}
