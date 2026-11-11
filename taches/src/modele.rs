// src/modele.rs
#[derive(Debug, Clone, PartialEq)]
pub struct Tache {
    pub titre: String,
    pub(crate) id: u32, // visible dans tout le crate, pas à l'extérieur
    terminee: bool,     // privé : accessible seulement dans ce module
}

impl Tache {
    pub fn new(id: u32, titre: impl Into<String>) -> Self {
        Self { titre: titre.into(), id, terminee: false }
    }

    pub fn terminer(&mut self) {
        self.terminee = true;
    }

    pub fn est_terminee(&self) -> bool {
        self.terminee
    }
}
