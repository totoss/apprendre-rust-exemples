use tracing::{debug, error, info, instrument, warn};

#[instrument(skip(contenu), fields(taille = contenu.len()))]
fn importer(nom: &str, contenu: &str) -> Result<usize, std::num::ParseIntError> {
    debug!("début de l'import");
    let mut total = 0;
    for ligne in contenu.lines() {
        total += ligne.trim().parse::<usize>()?;
    }
    info!(total, "import terminé");
    Ok(total)
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    if let Err(e) = importer("stock.txt", "12\n30\nabc") {
        error!(erreur = %e, "import impossible");
    }
    warn!("fin du programme");
}
