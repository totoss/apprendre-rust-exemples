use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

async fn servir(socket: TcpStream) -> std::io::Result<()> {
    let (lecture, mut ecriture) = socket.into_split();
    let mut lignes = BufReader::new(lecture).lines();
    while let Some(ligne) = lignes.next_line().await? {
        ecriture.write_all(ligne.to_uppercase().as_bytes()).await?;
        ecriture.write_all(b"\n").await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").await?;
    println!("écoute sur 127.0.0.1:7878");
    loop {
        let (socket, adresse) = ecouteur.accept().await?;
        tokio::spawn(async move { // une tâche par connexion
            if let Err(e) = servir(socket).await {
                eprintln!("{adresse} : {e}");
            }
        });
    }
}
