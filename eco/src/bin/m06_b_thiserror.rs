use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("lecture impossible : {0}")]
    Lecture(#[from] std::io::Error),
    #[error("port invalide : {0}")]
    PortInvalide(#[from] std::num::ParseIntError),
    #[error("le port {0} est réservé")]
    PortReserve(u16),
}

// --- harnais de vérification, hors cours : même logique que la version écrite à la main ---
fn lire_port(chemin: &str) -> Result<u16, ConfigError> {
    let contenu = std::fs::read_to_string(chemin)?;
    let port: u16 = contenu.trim().parse()?;
    if port < 1024 {
        return Err(ConfigError::PortReserve(port));
    }
    Ok(port)
}

fn main() {
    match lire_port("port.txt") {
        Ok(port) => println!("écoute sur le port {port}"),
        Err(e) => eprintln!("erreur : {e}"),
    }
}
