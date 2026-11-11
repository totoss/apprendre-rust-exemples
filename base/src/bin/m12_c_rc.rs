use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Compte {
    solde: i64,
}

fn main() {
    let compte = Rc::new(RefCell::new(Compte { solde: 100 }));
    let pour_la_carte = Rc::clone(&compte); // deuxième propriétaire, même donnée
    let pour_l_appli = Rc::clone(&compte);

    pour_la_carte.borrow_mut().solde -= 30; // emprunt mutable vérifié à l'exécution
    pour_l_appli.borrow_mut().solde += 5;

    println!("{} propriétaires, solde {}", Rc::strong_count(&compte), compte.borrow().solde);

    let lecture = compte.borrow();
    // let ecriture = compte.borrow_mut(); // panic : already borrowed
    println!("{lecture:?}");
}
