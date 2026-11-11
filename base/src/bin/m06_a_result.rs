use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    Lecture(io::Error),
    PortInvalide(ParseIntError),
    PortReserve(u16),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Lecture(e) => write!(f, "lecture impossible : {e}"),
            ConfigError::PortInvalide(e) => write!(f, "port invalide : {e}"),
            ConfigError::PortReserve(p) => write!(f, "le port {p} est réservé"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Lecture(e)
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::PortInvalide(e)
    }
}

fn lire_port(chemin: &str) -> Result<u16, ConfigError> {
    let contenu = fs::read_to_string(chemin)?; // io::Error devient ConfigError
    let port: u16 = contenu.trim().parse()?;   // ParseIntError devient ConfigError
    if port < 1024 {
        return Err(ConfigError::PortReserve(port));
    }
    Ok(port)
}

fn main() {
    match lire_port("port.txt") {
        Ok(port) => println!("écoute sur le port {port}"),
        Err(e) => {
            eprintln!("erreur : {e}");
            std::process::exit(1);
        }
    }
}
