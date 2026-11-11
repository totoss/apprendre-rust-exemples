# Exemples du cours « Apprendre Rust »

Chaque bloc de code du cours est recopié ici dans un fichier compilable, pour vérifier qu'il compile et pour te servir de point de départ.

| Dossier | Contenu | Dépendances externes |
| --- | --- | --- |
| `base/src/bin/` | Modules 2 à 19, un fichier par bloc de code | Aucune |
| `taches/` | Module 7 : bibliothèque, modules, binaire mince | Aucune |
| `notes/` | Module 10 : tests unitaires, d'intégration, doctest | Aucune |
| `nostd/` | Module 20 : bibliothèque `no_std` | Aucune |
| `ffi/` | Module 17 : fonction exposée à C | Aucune |
| `erreurs/` | Extraits qui doivent échouer, avec le code d'erreur attendu | Aucune |
| `eco/` | Modules 6, 13, 14, 15, 18, 19 : thiserror, anyhow, rayon, tokio, serde, clap, tracing, reqwest, criterion | Oui |
| `api-notes/` | Module 16 : API Axum et SQLx avec ses tests | Oui |
| `decrire_derive/` | Module 18 : macro procédurale | Oui |
| `rp2040-zero/` | Module 20 : firmware Embassy pour une carte RP2040-Zero | Oui, projet à part |

Les lignes qui suivent un commentaire `harnais de vérification, hors cours` ne figurent pas dans le cours : elles donnent un `main` aux extraits qui n'en ont pas.

## Exemples sans dépendance externe

Sans `--workspace`, Cargo ne traite que ces crates.

```
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -p base --bin m11_d_fibonacci
./erreurs/verifier.ps1
```

## Exemples avec dépendances externes

Tout d'un coup :

```
cargo test --workspace
cargo bench -p eco --no-run
```

Un exemple seul, avec ses seules dépendances (la liste est dans `eco/Cargo.toml`) :

```
cargo run -p eco --no-default-features --features tokio --bin m14_a_async
cargo run -p eco --no-default-features --features clap --bin m15_b_clap -- --help
```

## Firmware pour RP2040-Zero

`rp2040-zero/` ne fait pas partie du workspace : il se compile pour le microcontrôleur (`thumbv6m-none-eabi`), pas pour le PC. Il fait passer la LED RGB de la carte (WS2812, broche 16) au rouge, au vert, puis au bleu, toutes les demi-secondes.

Pour l'essayer sans rien installer : maintiens BOOT, appuie sur RESET, relâche, puis copie `rp2040-zero/rp2040-zero.uf2` sur la clé USB `RPI-RP2` qui apparaît. Ce fichier a été produit à partir de ces sources le 11 novembre 2026.

Pour le recompiler et le téléverser toi-même, depuis le dossier `rp2040-zero/` :

```
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs --locked
cargo run --release
```

Si la première couleur est verte au lieu de rouge, la LED de ta carte attend l'ordre RGB : le module 20 du cours indique la ligne à changer.

## Trois pièges rencontrés

- **Smart App Control.** S'il est activé, il refuse certains exécutables fraîchement compilés (`os error 4551`), y compris les scripts de build de dépendances. Sur le poste de vérification, ceux de serde et de crossbeam-utils étaient refusés à chaque essai, ce qui empêchait de compiler `api-notes` et quatre exemples de `eco` ; tout a compilé une fois Smart App Control désactivé. Le module 1 du cours décrit les deux voies possibles.
- **Journal muet.** Dans ce workspace, `eco` active la feature `env-filter` de `tracing-subscriber`, et Cargo l'applique alors aussi à `api-notes` : construit avec `--workspace`, le serveur n'affiche que les erreurs tant que `RUST_LOG` n'est pas défini. Construit seul (`cargo run -p api-notes`), il affiche bien « serveur démarré ».
- **Chemins longs.** L'éditeur de liens échoue avec `LNK1104` au-delà de 260 caractères environ. Si ce dossier est profond, déplace-le près de la racine ou définis `CARGO_TARGET_DIR` vers un dossier au chemin court.

## État au 11 novembre 2026

Vérifié avec Rust 1.99.1 et Visual Studio 18 sous Windows 11.

- Tout le workspace : `cargo check --workspace --all-targets` et Clippy strict sans avertissement, `cargo test --workspace` avec 11 tests réussis (doctest de `notes` et les deux tests de `api-notes` compris).
- Crates sans dépendance externe : rustdoc, les 22 extraits de `erreurs/`, et les 45 programmes de `base` exécutés avec des sorties conformes aux commentaires.
- `eco` : les 12 exemples exécutés, dont le serveur TCP interrogé en local et la requête HTTP de `m15_d_reqwest` vers l'API publique de GitHub. Le banc `comptage` a tourné : environ 280 µs pour la version naïve, 25 µs pour la version à itérateur.
- `api-notes` : serveur lancé et interrogé en local. Statuts observés : 201 à la création, 404 avec corps JSON, 422 pour un titre vide ou un champ manquant, 400 pour un JSON mal formé ou un identifiant non numérique, 415 pour un mauvais `Content-Type`.
- `rp2040-zero` : compile et se lie pour `thumbv6m-none-eabi` avec embassy-rp 0.10.0, passe Clippy strict, et `elf2uf2-rs` en tire une image UF2 valide (25 blocs, environ 6 Ko). Jamais exécuté sur une carte : la couleur et le rythme de la LED restent à constater.
