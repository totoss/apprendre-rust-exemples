use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Tache = Box<dyn FnOnce() + Send + 'static>;

fn main() {
    let (tx, rx) = mpsc::channel::<Tache>();
    let rx = Arc::new(Mutex::new(rx));

    let ouvriers: Vec<_> = (0..4)
        .map(|id| {
            let rx = Arc::clone(&rx);
            thread::spawn(move || loop {
                let tache = rx.lock().unwrap().recv(); // verrou rendu en fin d'instruction
                match tache {
                    Ok(tache) => tache(),
                    Err(_) => {
                        println!("ouvrier {id} : terminé");
                        break;
                    }
                }
            })
        })
        .collect();

    for n in 0..8 {
        tx.send(Box::new(move || println!("tâche {n}"))).unwrap();
    }
    drop(tx); // ferme le canal : les ouvriers sortent de leur boucle

    for o in ouvriers {
        o.join().unwrap();
    }
}
