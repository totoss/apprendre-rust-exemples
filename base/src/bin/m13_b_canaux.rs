use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    for id in 0..3 {
        let tx = tx.clone(); // un émetteur par thread
        thread::spawn(move || {
            for i in 0..2 {
                tx.send(format!("ouvrier {id} : tâche {i}")).unwrap();
            }
        }); // tx est libéré à la fin du thread
    }
    drop(tx); // libère l'émetteur d'origine

    for message in rx { // s'arrête quand tous les émetteurs ont disparu
        println!("{message}");
    }
}
