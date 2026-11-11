//! Outils de calcul sur des notes.

/// Calcule la moyenne d'une série de notes.
///
/// Renvoie `None` si la série est vide.
///
/// # Examples
///
/// ```
/// use notes::moyenne;
/// assert_eq!(moyenne(&[10.0, 14.0]), Some(12.0));
/// assert_eq!(moyenne(&[]), None);
/// ```
pub fn moyenne(notes: &[f64]) -> Option<f64> {
    if notes.is_empty() {
        return None;
    }
    Some(notes.iter().sum::<f64>() / notes.len() as f64)
}

/// Renvoie la mention de la moyenne, ou `None` si la série est vide.
pub fn mention_de(notes: &[f64]) -> Option<&'static str> {
    moyenne(notes).map(mention)
}

fn mention(note: f64) -> &'static str {
    match note {
        n if n >= 16.0 => "très bien",
        n if n >= 14.0 => "bien",
        n if n >= 12.0 => "assez bien",
        n if n >= 10.0 => "passable",
        _ => "insuffisant",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moyenne_de_deux_notes() {
        assert_eq!(moyenne(&[10.0, 14.0]), Some(12.0));
    }

    #[test]
    fn moyenne_d_une_serie_vide() {
        assert_eq!(moyenne(&[]), None);
    }

    #[test]
    fn mentions_aux_bornes() {
        assert_eq!(mention(16.0), "très bien");
        assert_eq!(mention(9.99), "insuffisant");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn indexer_un_vecteur_vide_panique() {
        let vide: Vec<i32> = Vec::new();
        let _ = vide[0];
    }

    #[test]
    fn un_test_peut_renvoyer_un_result() -> Result<(), std::num::ParseIntError> {
        let n: i32 = "42".parse()?;
        assert_eq!(n, 42);
        Ok(())
    }
}
