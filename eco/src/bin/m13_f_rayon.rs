use rayon::prelude::*;

fn main() {
    let nombres: Vec<u64> = (1..=5_000_000).collect();
    let somme: u64 = nombres.par_iter().map(|n| n * n % 1_000).sum();
    println!("{somme}");
}
