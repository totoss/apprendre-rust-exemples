macro_rules! table {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($cle:expr => $valeur:expr),+ $(,)?) => {{
        let mut t = ::std::collections::HashMap::new();
        $( t.insert($cle, $valeur); )+
        t
    }};
}

fn main() {
    let ages = table! { "ada" => 36, "linus" => 54, };
    let vide: std::collections::HashMap<String, u8> = table!();
    println!("{ages:?} {}", vide.len());
}
