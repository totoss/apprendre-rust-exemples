struct Chrono {
    nom: &'static str,
    debut: std::time::Instant,
}

impl Drop for Chrono {
    fn drop(&mut self) {
        println!("{} : {:?}", self.nom, self.debut.elapsed());
    }
}

fn main() {
    let _chrono = Chrono { nom: "calcul", debut: std::time::Instant::now() };
    let somme: u64 = (1..=1_000_000).sum();
    println!("{somme}");
} // _chrono est libéré ici : drop affiche la durée
