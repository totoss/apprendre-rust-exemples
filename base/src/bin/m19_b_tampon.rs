use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
    let mut entree = io::stdin().lock();
    let mut sortie = BufWriter::new(io::stdout().lock()); // écrit par blocs
    let mut tampon = String::with_capacity(256);
    let mut total = 0usize;

    while entree.read_line(&mut tampon)? != 0 {
        total += tampon.len();
        sortie.write_all(tampon.to_uppercase().as_bytes())?;
        tampon.clear(); // garde la capacité : aucune réallocation au tour suivant
    }
    writeln!(sortie, "{total} octets")?;
    Ok(())
}
