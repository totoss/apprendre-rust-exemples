# Parcours « Apprendre Rust » : exemples du cours

Ce dépôt contient les exemples vérifiés d'un cours de Rust que son propriétaire suit en autodidacte. Ce fichier donne le contexte qui ne se déduit pas du code, pour qu'une session ouverte sur n'importe quel poste reparte du même point.

## La personne

- Développeur venant d'un langage de haut niveau (Python, JavaScript, Java), sans pratique de la gestion manuelle de la mémoire.
- Vise quatre usages de Rust : généraliste, backend web et API, outils en ligne de commande, système et embarqué.
- Parcours de 18 semaines commencé le 11 novembre 2026, à raison de 6 à 10 heures par semaine.
- Échanges en français, en tutoyant. Les références en anglais conviennent.
- Travaille sur plusieurs postes. Ce dépôt et les deux documents ci-dessous sont ce qu'ils ont en commun.

## Les documents du parcours

Ils vivent sur claude.ai, pas dans ce dépôt, et ne s'ouvrent qu'avec le compte Claude du propriétaire. Ils se lisent avec les outils d'artifacts et de documents de Claude, pas par une requête web.

- Cours complet, en neuf onglets (vue d'ensemble et plan, cinq parties, Projets, Évaluation, Références) : https://claude.ai/artifact/HMxc4w9svD1KXTmTzeDaUD
- Kanban de suivi, une carte par module M1 à M20 et par projet P1 à P5 : https://claude.ai/artifact/T5UFpJLCZ18FZV6pzjX1fi

Le cours fait foi. Si une session n'y a pas accès, dis-le et demande le passage utile plutôt que de le reconstituer de mémoire.

## Le plan

Cinq parties (Fondations, Rust idiomatique, Intermédiaire, Rust appliqué, Avancé et système), vingt modules, cinq projets jalons.

| Projet | Sujet | Nom du crate | Après le module |
| --- | --- | --- | --- |
| P1 | Calculatrice en notation polonaise inverse | `rpn` | 4 |
| P2 | Gestionnaire de tâches en ligne de commande | `tache` | 10 |
| P3 | Détecteur de fichiers en double | `doublons` | 13 |
| P4 | Service de liens courts | `raccourci` | 16 |
| P5 | Moteur clé-valeur réseau, ou station de mesure embarquée | `kv` | 20 |

Les projets sont écrits par l'apprenant, chacun dans son propre dépôt. Ce dépôt-ci ne contient que les exemples du cours.

## Évaluer un projet

L'onglet « Évaluation » du cours contient les instructions complètes et le barème ; l'onglet « Projets » contient le cahier des charges de chaque projet. Lis-les avant d'évaluer. En résumé :

- Tu es un relecteur exigeant. Tu ne modifies aucun fichier du projet et tu n'écris pas la solution : un constat, sa preuve (fichier et ligne, sortie de commande ou test), une piste, le module à relire. Cinq lignes d'extrait au plus.
- Tu n'évalues que ce qui a été enseigné à la semaine du projet.
- Tu exécutes vraiment `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo doc --no-deps` (à partir de P2) et `cargo build --release`, puis tu essaies de casser le binaire avec des entrées hostiles.
- Barème sur 100 : conformité 30, Rust idiomatique 20, gestion d'erreurs 15, tests 15, conception 10, outillage et documentation 10.
- Verdict VALIDÉ à 70 ou plus sans point bloquant, sinon À REPRENDRE. Le rapport suit les neuf sections fixées par le cours.
- Le barème est volontairement strict : n'arrondis jamais vers le haut par gentillesse.

## Ce dépôt

Un fichier compilable par bloc de code du cours. Le détail des dossiers, les commandes et l'état de la vérification sont dans `README.md`.

- Workspace Cargo à la racine. Sans `--workspace`, Cargo ne traite que les crates sans dépendance externe (`default-members`).
- `rp2040-zero/` est un projet à part, compilé pour `thumbv6m-none-eabi`.
- `erreurs/` contient des extraits qui doivent échouer ; `erreurs/verifier.ps1` contrôle leurs codes d'erreur.
- Dans un exemple, ce qui suit le commentaire `harnais de vérification, hors cours` n'existe pas dans le cours.

Pour modifier un exemple :

- Le fichier et le bloc de code correspondant du cours doivent rester identiques. Si tu changes l'un, change l'autre, ou signale l'écart.
- Avant d'écrire qu'un exemple est vérifié, lance `cargo check --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings` et `cargo test --workspace`, puis exécute le programme concerné. Dis clairement ce qui n'a pas été exécuté.
- Les extraits du cours sont volontairement compacts : `cargo fmt --check` signale des écarts, ce n'est pas une erreur ici.
- Quand l'état de la vérification change, mets à jour la section « État » de `README.md` et le paragraphe « État des exemples » de la vue d'ensemble du cours.

## Matériel embarqué

Le propriétaire a des clones de Waveshare RP2040-Zero : un RP2040, une LED RGB WS2812 sur la broche 16, pas de broches SWD (donc ni sonde ni `defmt-rtt`), téléversement par fichier UF2 après BOOT puis RESET. Vise cette carte pour le module 20 et pour la variante embarquée de P5.

Au 12 novembre 2026, le firmware de `rp2040-zero/` compile et donne une image UF2 valide, mais n'a pas encore tourné sur une carte. L'ordre des couleurs de la LED (GRB ou RGB) reste à constater.

## Pièges déjà rencontrés

- Smart App Control de Windows 11, s'il est activé, bloque des exécutables compilés par Cargo et certains scripts de build (`os error 4551`). C'est un réglage de sécurité qui appartient au propriétaire : signale-le, n'y touche pas et ne cherche pas à le contourner.
- L'éditeur de liens de Visual Studio échoue avec `LNK1104` quand un chemin dépasse 260 caractères environ. Garde les projets près de la racine du disque, ou définis `CARGO_TARGET_DIR`.
- `tracing_subscriber::fmt::init()` n'affiche que les erreurs quand la feature `env-filter` est activée et que `RUST_LOG` n'est pas défini. Dans ce workspace, c'est le cas d'`api-notes` construit avec `--workspace`.
- Embassy change vite. Pour `rp2040-zero`, fie-toi au code source de la version figée dans `Cargo.lock`, pas aux exemples de la branche principale du dépôt Embassy.
- Sur un poste Windows, `cargo`, `git` et `gh` peuvent être installés sans figurer dans le `PATH` de la session : cherche-les à leur emplacement habituel avant de conclure qu'ils manquent.

## Git

- Ce dépôt est privé. Garde les nouveaux dépôts privés sauf demande contraire.
- Commits au nom du profil GitHub du propriétaire, avec son adresse `noreply`. Son adresse personnelle ne doit apparaître ni dans un commit ni dans un fichier.
- Messages de commit en français.
