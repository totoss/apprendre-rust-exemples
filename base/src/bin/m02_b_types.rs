fn main() {
    let a: i32 = -42;
    let b: u8 = 255;
    let d = 1_000_000u64;           // suffixe de type, séparateurs libres

    let large = a as i64;           // conversion explicite, peut tronquer
    let sur = i64::from(a);         // conversion garantie sans perte
    let risque = u8::try_from(300); // Err : 300 ne tient pas dans un u8
    println!("{large} {sur} {d} {risque:?}");

    let somme = b.checked_add(1);    // None : dépassement détecté
    let borne = b.saturating_add(1); // 255
    let tour = b.wrapping_add(1);    // 0
    println!("{somme:?} {borne} {tour}");
}
