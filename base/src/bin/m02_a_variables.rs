fn main() {
    let x = 5;             // immuable par défaut
    let mut compteur = 0;  // mutable, explicitement
    compteur += x;

    let x = x * 2;         // shadowing : une nouvelle variable masque l'ancienne
    let x = x.to_string(); // et peut même changer de type
    println!("{x} {compteur}");

    const SECONDES_PAR_JOUR: u32 = 24 * 60 * 60;
    println!("{SECONDES_PAR_JOUR}");
}
