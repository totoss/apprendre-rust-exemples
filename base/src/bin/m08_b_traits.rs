trait Resume {
    fn auteur(&self) -> String;

    fn resume(&self) -> String { // méthode par défaut
        format!("(lire la suite de {}...)", self.auteur())
    }
}

struct Article {
    titre: String,
    auteur: String,
}

struct Message {
    pseudo: String,
}

impl Resume for Article {
    fn auteur(&self) -> String {
        self.auteur.clone()
    }

    fn resume(&self) -> String {
        format!("{}, par {}", self.titre, self.auteur)
    }
}

impl Resume for Message {
    fn auteur(&self) -> String {
        format!("@{}", self.pseudo)
    }
}

// Dispatch statique : une version compilée par type concret
fn annoncer(element: &impl Resume) {
    println!("Nouveau : {}", element.resume());
}

// Dispatch dynamique : types mélangés, méthode choisie à l'exécution
fn tout_annoncer(elements: &[Box<dyn Resume>]) {
    for e in elements {
        println!("{}", e.resume());
    }
}

fn main() {
    let article = Article { titre: String::from("Rust 2024"), auteur: String::from("Ada") };
    let message = Message { pseudo: String::from("ferris") };
    annoncer(&article);
    let flux: Vec<Box<dyn Resume>> = vec![Box::new(article), Box::new(message)];
    tout_annoncer(&flux);
}
