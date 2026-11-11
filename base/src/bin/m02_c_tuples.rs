fn main() {
    let point = (3, -1.5);       // tuple : types hétérogènes, taille fixe
    let (x, y) = point;          // déstructuration
    let premier = point.0;

    let zeros = [0u8; 4];        // tableau : même type, taille connue à la compilation
    let jours = ["lun", "mar", "mer"];
    println!("{x} {y} {premier} {zeros:?} {}", jours.len());
}
