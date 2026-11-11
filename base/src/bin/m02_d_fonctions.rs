fn aire(largeur: u32, hauteur: u32) -> u32 {
    largeur * hauteur // pas de point-virgule : c'est la valeur de retour
}

fn categorie(age: u32) -> &'static str {
    if age < 18 { "mineur" } else { "majeur" }
}

fn main() {
    let note = {
        let brut = aire(3, 4);
        brut + 1 // un bloc vaut sa dernière expression
    };
    println!("{note} {}", categorie(20));
}
