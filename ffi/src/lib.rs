use std::ffi::{CStr, c_char};

#[repr(C)]
pub struct Statistiques {
    pub lignes: u32,
    pub mots: u32,
}

/// Compte les lignes et les mots d'un texte.
///
/// # Safety
///
/// `texte` doit être nul ou pointer vers une chaîne C valide, terminée par un nul.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn compter(texte: *const c_char) -> Statistiques {
    if texte.is_null() {
        return Statistiques { lignes: 0, mots: 0 };
    }
    // SAFETY : pointeur non nul vérifié ci-dessus, validité garantie par le contrat.
    let texte = unsafe { CStr::from_ptr(texte) }.to_string_lossy();
    Statistiques {
        lignes: texte.lines().count() as u32,
        mots: texte.split_whitespace().count() as u32,
    }
}
