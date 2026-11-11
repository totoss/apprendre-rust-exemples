use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Etat {
    AFaire,
    EnCours,
    Terminee,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Tache {
    id: u32,
    titre: String,
    etat: Etat,
    #[serde(default)]
    etiquettes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    echeance: Option<String>,
}

fn main() -> Result<(), serde_json::Error> {
    let json = r#"{ "id": 1, "titre": "apprendre serde", "etat": "en_cours" }"#;
    let tache: Tache = serde_json::from_str(json)?;
    println!("{tache:?}");
    println!("{}", serde_json::to_string_pretty(&tache)?);
    Ok(())
}
