use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Noeud {
    nom: String,
    parent: RefCell<Weak<Noeud>>,     // faible : ne maintient pas le parent en vie
    enfants: RefCell<Vec<Rc<Noeud>>>, // fort : le parent possède ses enfants
}

fn main() {
    let racine = Rc::new(Noeud {
        nom: String::from("racine"),
        parent: RefCell::new(Weak::new()),
        enfants: RefCell::new(Vec::new()),
    });
    let feuille = Rc::new(Noeud {
        nom: String::from("feuille"),
        parent: RefCell::new(Rc::downgrade(&racine)),
        enfants: RefCell::new(Vec::new()),
    });
    racine.enfants.borrow_mut().push(Rc::clone(&feuille));

    if let Some(parent) = feuille.parent.borrow().upgrade() {
        println!("{} a pour parent {}", feuille.nom, parent.nom);
    }
}
