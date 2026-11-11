#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

impl Rectangle {
    fn new(largeur: f64, hauteur: f64) -> Self {
        Self { largeur, hauteur } // raccourci : champ et variable de même nom
    }

    fn carre(cote: f64) -> Self {
        Self::new(cote, cote)
    }

    fn aire(&self) -> f64 {              // lit
        self.largeur * self.hauteur
    }

    fn agrandir(&mut self, facteur: f64) { // modifie
        self.largeur *= facteur;
        self.hauteur *= facteur;
    }
}

fn main() {
    let mut r = Rectangle::carre(2.0);
    r.agrandir(1.5);
    println!("{r:?} aire = {}", r.aire());
}
