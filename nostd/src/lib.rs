#![cfg_attr(not(test), no_std)]

/// Moyenne glissante sur N échantillons, sans allocation. N doit être supérieur à zéro.
pub struct MoyenneGlissante<const N: usize> {
    echantillons: [i32; N],
    position: usize,
    remplis: usize,
}

impl<const N: usize> Default for MoyenneGlissante<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> MoyenneGlissante<N> {
    pub const fn new() -> Self {
        Self { echantillons: [0; N], position: 0, remplis: 0 }
    }

    pub fn ajouter(&mut self, valeur: i32) -> i32 {
        self.echantillons[self.position] = valeur;
        self.position = (self.position + 1) % N;
        if self.remplis < N {
            self.remplis += 1;
        }
        let somme: i64 = self.echantillons[..self.remplis]
            .iter()
            .map(|&v| i64::from(v))
            .sum();
        (somme / self.remplis as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moyenne_sur_trois_echantillons() {
        let mut m = MoyenneGlissante::<3>::new();
        assert_eq!(m.ajouter(3), 3);
        assert_eq!(m.ajouter(5), 4);
        assert_eq!(m.ajouter(10), 6);
        assert_eq!(m.ajouter(0), 5); // (0 + 5 + 10) / 3
    }
}
