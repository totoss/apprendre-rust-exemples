fn premier_mot(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let phrase = String::from("bonjour le monde");
    let mot = premier_mot(&phrase); // &String se convertit en &str
    let debut = &phrase[0..7];      // slice de chaîne
    let nombres = [1, 2, 3, 4, 5];
    let milieu = &nombres[1..4];    // slice de tableau : &[i32]
    println!("{mot} {debut} {milieu:?}");
}
