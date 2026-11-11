fn appliquer_deux_fois(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn compteur() -> impl FnMut() -> u32 {
    let mut n = 0;
    move || {
        n += 1;
        n
    }
}

fn main() {
    let seuil = 10;
    let depasse = |x: i32| x > seuil; // capture seuil par référence
    println!("{}", depasse(12));

    let mut journal = Vec::new();
    let mut noter = |msg: &str| journal.push(msg.to_string()); // capture mutable
    noter("début");
    noter("fin");
    println!("{journal:?}");

    let nom = String::from("Ada");
    let saluer = move || println!("Bonjour {nom}"); // move : la closure possède nom
    saluer();

    println!("{}", appliquer_deux_fois(|x| x * 3, 2)); // 18
    let mut suivant = compteur();
    println!("{} {}", suivant(), suivant()); // 1 2
}
