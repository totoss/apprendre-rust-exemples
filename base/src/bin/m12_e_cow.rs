use std::borrow::Cow;

fn sans_espaces(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_")) // allocation seulement dans ce cas
    } else {
        Cow::Borrowed(s)
    }
}

// --- harnais de vérification, hors cours ---
fn main() {
    println!("{} {}", sans_espaces("a b c"), sans_espaces("abc"));
}
