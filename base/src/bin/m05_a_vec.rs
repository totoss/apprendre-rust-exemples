fn main() {
    let mut notes: Vec<u32> = Vec::new();
    notes.push(12);
    notes.push(15);
    notes.extend([9, 18]);
    let mut autres = vec![3, 1, 2];

    let premier = notes[0];     // panique si l'indice est hors bornes
    let dixieme = notes.get(9); // Option<&u32> : None ici
    println!("{premier} {dixieme:?} {}", notes.len());

    for n in &notes { print!("{n} "); } // lecture
    for n in &mut notes { *n += 1; }    // modification en place

    autres.sort();
    notes.retain(|n| *n >= 10);         // garde les éléments qui passent le test
    let total: u32 = notes.iter().sum();
    let max = notes.iter().max();       // Option<&u32>
    println!("{notes:?} {autres:?} {total} {max:?}");
}
