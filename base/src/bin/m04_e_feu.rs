#[derive(Debug, Clone, Copy, PartialEq)]
enum Feu {
    Vert,
    Orange,
    Rouge,
}

impl Feu {
    fn suivant(self) -> Self {
        match self {
            Feu::Vert => Feu::Orange,
            Feu::Orange => Feu::Rouge,
            Feu::Rouge => Feu::Vert,
        }
    }
}

// --- harnais de vérification, hors cours ---
fn main() {
    println!("{:?}", Feu::Vert.suivant().suivant().suivant());
}
