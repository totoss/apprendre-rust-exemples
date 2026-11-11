// tests/api.rs : un crate à part, qui ne voit que l'API publique
use notes::moyenne;

#[test]
fn moyenne_de_trois_notes() {
    assert_eq!(moyenne(&[8.0, 12.0, 16.0]), Some(12.0));
}
