#[derive(Debug)]
enum Commande {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    Couleur(u8, u8, u8),
}

fn executer(c: &Commande) -> String {
    match c {
        Commande::Quitter => String::from("au revoir"),
        Commande::Deplacer { x, y: 0 } => format!("déplacement horizontal de {x}"),
        Commande::Deplacer { x, y } => format!("déplacement vers ({x}, {y})"),
        Commande::Ecrire(texte) if texte.is_empty() => String::from("rien à écrire"),
        Commande::Ecrire(texte) => format!("écrit : {texte}"),
        Commande::Couleur(r, v, b) => format!("couleur ({r}, {v}, {b})"),
    }
}

// --- harnais de vérification, hors cours ---
fn main() {
    for c in [
        Commande::Quitter,
        Commande::Deplacer { x: 3, y: 0 },
        Commande::Deplacer { x: 1, y: 2 },
        Commande::Ecrire(String::new()),
        Commande::Ecrire(String::from("abc")),
        Commande::Couleur(255, 0, 0),
    ] {
        println!("{c:?} -> {}", executer(&c));
    }
}
