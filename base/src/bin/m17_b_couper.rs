fn couper_en_deux(tranche: &mut [i32], milieu: usize) -> (&mut [i32], &mut [i32]) {
    let longueur = tranche.len();
    let pointeur = tranche.as_mut_ptr();
    assert!(milieu <= longueur);
    // SAFETY : milieu <= longueur, donc les deux tranches restent dans les bornes
    // et ne se chevauchent pas ; elles héritent de l'emprunt exclusif de `tranche`.
    unsafe {
        (
            std::slice::from_raw_parts_mut(pointeur, milieu),
            std::slice::from_raw_parts_mut(pointeur.add(milieu), longueur - milieu),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let (gauche, droite) = couper_en_deux(&mut v, 2);
    gauche[0] = 10;
    droite[0] = 30;
    println!("{v:?}"); // [10, 2, 30, 4, 5]
}
