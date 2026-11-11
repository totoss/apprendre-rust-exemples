// tests/cli.rs : lance le binaire compilé comme le ferait un utilisateur
use std::process::Command;

#[test]
fn affiche_la_moyenne_des_arguments() {
    let sortie = Command::new(env!("CARGO_BIN_EXE_notes"))
        .args(["10", "14"])
        .output()
        .expect("le binaire doit se lancer");
    assert!(sortie.status.success());
    assert_eq!(String::from_utf8_lossy(&sortie.stdout).trim(), "12");
}
