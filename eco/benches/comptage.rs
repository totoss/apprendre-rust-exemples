// benches/comptage.rs
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn compter_mots_naif(texte: &str) -> usize {
    let mots: Vec<String> = texte.split(' ').map(|m| m.to_string()).collect();
    mots.iter().filter(|m| !m.is_empty()).count()
}

fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

fn bancs(c: &mut Criterion) {
    let texte = "le rapide renard brun saute par-dessus le chien paresseux ".repeat(1_000);
    c.bench_function("naif", |b| b.iter(|| compter_mots_naif(black_box(&texte))));
    c.bench_function("iterateur", |b| b.iter(|| compter_mots(black_box(&texte))));
}

criterion_group!(benches, bancs);
criterion_main!(benches);
