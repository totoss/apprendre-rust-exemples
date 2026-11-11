fn main() {
    let mut n = 27u64;
    let mut etapes = 0;
    let duree = loop {
        if n == 1 {
            break etapes; // loop peut renvoyer une valeur
        }
        n = if n.is_multiple_of(2) { n / 2 } else { 3 * n + 1 };
        etapes += 1;
    };
    println!("{duree}");

    for i in 0..3 { println!("{i}"); }          // 0, 1, 2
    for i in (1..=3).rev() { println!("{i}"); } // 3, 2, 1

    let notes = [12, 15, 9];
    for (rang, note) in notes.iter().enumerate() {
        println!("{rang}: {note}");
    }

    'externe: for x in 0..10 {
        for y in 0..10 {
            if x * y > 20 { break 'externe; } // sort des deux boucles
        }
    }
}
