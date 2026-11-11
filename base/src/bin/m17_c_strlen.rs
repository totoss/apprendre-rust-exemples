use std::ffi::{CString, c_char, c_int};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    safe fn abs(valeur: c_int) -> c_int; // déclarée sûre : aucune précondition
}

fn longueur_c(texte: &str) -> Option<usize> {
    let c = CString::new(texte).ok()?; // échoue si le texte contient un octet nul
    // SAFETY : c est une chaîne C valide, terminée par un nul, vivante pendant l'appel.
    Some(unsafe { strlen(c.as_ptr()) })
}

fn main() {
    println!("{:?} {}", longueur_c("bonjour"), abs(-3));
}
