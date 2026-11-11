use clap::{Parser, Subcommand};

/// Gestionnaire de tâches en ligne de commande
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Fichier de données
    #[arg(short, long, default_value = "taches.json", env = "TACHES_FICHIER")]
    fichier: std::path::PathBuf,

    /// Augmente le niveau de détail (-v, -vv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbeux: u8,

    #[command(subcommand)]
    commande: Commande,
}

#[derive(Subcommand, Debug)]
enum Commande {
    /// Ajoute une tâche
    Ajouter {
        titre: String,
        #[arg(short, long, default_value_t = 3)]
        priorite: u8,
    },
    /// Liste les tâches
    Lister {
        #[arg(long)]
        toutes: bool,
    },
    /// Marque une tâche comme terminée
    Terminer { id: u32 },
}

fn main() {
    let cli = Cli::parse();
    println!("{cli:?}");
}
