fn premier_mot<'a>(texte: &'a str, _separateurs: &str) -> &'a str {
    texte.split_whitespace().next().unwrap_or("")
}

// --- harnais de vérification, hors cours ---
fn main() {
    let mot;
    {
        let separateurs = String::from(" ");
        mot = premier_mot("bonjour le monde", &separateurs);
    }
    println!("{mot}"); // le résultat ne dépend pas de la durée de vie de `separateurs`
}
