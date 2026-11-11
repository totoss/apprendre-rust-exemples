fn analyser(ligne: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    ligne.split(',').map(|s| s.trim().parse::<i32>()).collect()
}

fn main() {
    println!("{:?}", analyser("1, 2, 3")); // Ok([1, 2, 3])
    println!("{:?}", analyser("1, x"));    // Err : s'arrête à la première erreur

    let carres_pairs: Vec<u64> = (1..) // itérateur infini
        .map(|n| n * n)
        .filter(|n| n % 2 == 0)
        .take(3)
        .collect(); // [4, 16, 36]
    let initiales: String = ["ada", "bob"].iter().filter_map(|m| m.chars().next()).collect();
    println!("{carres_pairs:?} {initiales}");
}
