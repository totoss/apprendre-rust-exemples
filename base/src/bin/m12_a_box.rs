#[derive(Debug)]
enum Expr {
    Nombre(f64),
    Addition(Box<Expr>, Box<Expr>),
    Produit(Box<Expr>, Box<Expr>),
}

fn evaluer(e: &Expr) -> f64 {
    match e {
        Expr::Nombre(n) => *n,
        Expr::Addition(a, b) => evaluer(a) + evaluer(b),
        Expr::Produit(a, b) => evaluer(a) * evaluer(b),
    }
}

fn main() {
    // (2 + 3) * 4
    let expr = Expr::Produit(
        Box::new(Expr::Addition(
            Box::new(Expr::Nombre(2.0)),
            Box::new(Expr::Nombre(3.0)),
        )),
        Box::new(Expr::Nombre(4.0)),
    );
    println!("{}", evaluer(&expr)); // 20
}
