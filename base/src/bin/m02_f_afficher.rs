fn main() {
    let nom = "Ada";
    let age = 36;
    println!("{nom} a {age} ans, soit {} mois", age * 12);
    println!("{:>8.2}|{:<5}|{:05}", 7.12345, "ab", 42); // "    7.12|ab   |00042"
    let paire = (1, "un");
    println!("{paire:?}");  // format Debug, pour le développeur
    dbg!(age * 2);          // affiche fichier, ligne, expression et valeur
}
