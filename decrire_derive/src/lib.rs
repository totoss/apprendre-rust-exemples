// Dans un crate à part, avec `proc-macro = true` sous [lib] dans Cargo.toml
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Decrire)]
pub fn derive_decrire(entree: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(entree as DeriveInput);
    let nom = &ast.ident;
    let genere = quote! {
        impl Decrire for #nom {
            fn decrire() -> &'static str {
                stringify!(#nom)
            }
        }
    };
    genere.into()
}
