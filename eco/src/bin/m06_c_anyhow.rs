use anyhow::{Context, Result, bail};

fn lire_port(chemin: &str) -> Result<u16> {
    let contenu = std::fs::read_to_string(chemin)
        .with_context(|| format!("lecture de {chemin}"))?;
    let port: u16 = contenu.trim().parse().context("port invalide")?;
    if port < 1024 {
        bail!("le port {port} est réservé");
    }
    Ok(port)
}

fn main() -> Result<()> {
    let port = lire_port("port.txt")?;
    println!("écoute sur le port {port}");
    Ok(())
}
