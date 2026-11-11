#[derive(Debug)]
struct Extrait<'a> {
    texte: &'a str,
}

impl<'a> Extrait<'a> {
    fn premiere_phrase(roman: &'a str) -> Option<Self> {
        let texte = roman.split('.').next()?;
        Some(Extrait { texte })
    }

    fn longueur(&self) -> usize { // élision : aucune annotation
        self.texte.len()
    }
}

fn main() {
    let roman = String::from("Appelez-moi Ismaël. Il y a quelques années...");
    let extrait = Extrait::premiere_phrase(&roman).expect("au moins une phrase");
    println!("{extrait:?} {}", extrait.longueur());
}
