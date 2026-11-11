fn plus_grand<T: PartialOrd>(elements: &[T]) -> Option<&T> {
    let mut max = elements.first()?;
    for e in elements {
        if e > max {
            max = e;
        }
    }
    Some(max)
}

#[derive(Debug)]
struct Paire<T> {
    gauche: T,
    droite: T,
}

impl<T: Clone> Paire<T> {
    fn inversee(&self) -> Paire<T> {
        Paire { gauche: self.droite.clone(), droite: self.gauche.clone() }
    }
}

fn main() {
    println!("{:?}", plus_grand(&[3, 9, 4]));
    println!("{:?}", plus_grand(&["kiwi", "pomme"]));
    println!("{:?}", Paire { gauche: 1, droite: 2 }.inversee());
}
