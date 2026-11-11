fn main() {
    let mut s = String::from("a");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}"); // dernière utilisation de r1 et r2
    let r3 = &mut s;       // accepté : plus aucun emprunt partagé actif
    r3.push('b');
    println!("{r3}");
}
