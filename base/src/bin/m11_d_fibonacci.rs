struct Fibonacci {
    courant: u64,
    suivant: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let resultat = self.courant;
        let nouveau = self.courant.checked_add(self.suivant)?; // None au dépassement : fin
        self.courant = self.suivant;
        self.suivant = nouveau;
        Some(resultat)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { courant: 0, suivant: 1 }
}

fn mots_longs(texte: &str, min: usize) -> impl Iterator<Item = &str> {
    texte.split_whitespace().filter(move |m| m.chars().count() >= min)
}

fn main() {
    let premiers: Vec<u64> = fibonacci().take(10).collect();
    let somme_paire: u64 = fibonacci()
        .take_while(|n| *n < 4_000_000)
        .filter(|n| n % 2 == 0)
        .sum();
    println!("{premiers:?} {somme_paire}"); // ... 4613732
    println!("{:?}", mots_longs("un itérateur paresseux", 5).collect::<Vec<_>>());
}
