fn diviser(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    match diviser(10.0, 4.0) {
        Some(q) => println!("quotient : {q}"),
        None => println!("division par zéro"),
    }

    if let Some(q) = diviser(1.0, 3.0) { // un seul cas m'intéresse
        println!("{q:.3}");
    }

    let Some(q) = diviser(9.0, 3.0) else { // sortie anticipée sinon
        return;
    };

    let double = diviser(1.0, 0.0).map(|q| q * 2.0).unwrap_or(0.0);
    println!("{q} {double}");
}
