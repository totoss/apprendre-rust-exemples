fn main() {
    let mut x = 10;
    let p: *mut i32 = &raw mut x; // créer un pointeur brut est sûr
    // SAFETY : p vient d'une variable vivante et alignée, et rien d'autre n'y accède.
    unsafe {
        *p += 1; // le déréférencer exige unsafe
        println!("{}", *p);
    }
}
