use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Depot {
    full_name: String,
    stargazers_count: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::builder()
        .user_agent("cours-rust")
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let depot: Depot = client
        .get("https://api.github.com/repos/rust-lang/rust")
        .send()
        .await?
        .error_for_status()? // transforme un statut 4xx ou 5xx en erreur
        .json()
        .await?;

    println!("{} : {} étoiles", depot.full_name, depot.stargazers_count);
    Ok(())
}
