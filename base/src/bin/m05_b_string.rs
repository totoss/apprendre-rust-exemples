fn main() {
    let litteral: &str = "été";         // &'static str, stocké dans le binaire
    let mut s = String::from(litteral); // String : possédée, modifiable
    s.push_str(" indien");
    s.push('!');
    let complet = format!("{s} ({} octets)", s.len());
    println!("{complet}");

    println!("{}", "été".len());           // 5 : des octets, pas des caractères
    println!("{}", "été".chars().count()); // 3
    // let c = s[0];                       // erreur E0277 : pas d'indexation par entier
    let premier = s.chars().next();        // Some('é')
    for (i, c) in "été".char_indices() {   // (0, 'é'), (2, 't'), (3, 'é')
        println!("{i} {c}");
    }

    let ligne = "  nom = Ada ; age = 36  ";
    let champs: Vec<&str> = ligne.trim().split(';').map(str::trim).collect();
    let age: u32 = "36".parse().unwrap_or(0);
    println!("{champs:?} {age} {premier:?}");
}
