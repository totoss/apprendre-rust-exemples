use std::collections::HashMap;

#[derive(Debug)]
struct Commande {
    client: String,
    montant: f64,
    payee: bool,
}

fn main() {
    let commandes = vec![
        Commande { client: "ada".into(), montant: 120.0, payee: true },
        Commande { client: "linus".into(), montant: 80.0, payee: false },
        Commande { client: "ada".into(), montant: 45.5, payee: true },
    ];

    let total_paye: f64 = commandes
        .iter()
        .filter(|c| c.payee)
        .map(|c| c.montant)
        .sum();

    let impayes: Vec<&str> = commandes
        .iter()
        .filter(|c| !c.payee)
        .map(|c| c.client.as_str())
        .collect();

    let plus_grosse = commandes
        .iter()
        .max_by(|a, b| a.montant.total_cmp(&b.montant));

    let (payees, en_attente): (Vec<&Commande>, Vec<&Commande>) =
        commandes.iter().partition(|c| c.payee);

    let mut par_client: HashMap<&str, f64> = HashMap::new();
    for c in &commandes {
        *par_client.entry(c.client.as_str()).or_default() += c.montant;
    }

    println!("{total_paye} {impayes:?} {par_client:?}");
    println!("{plus_grosse:?} {} {}", payees.len(), en_attente.len());
}
