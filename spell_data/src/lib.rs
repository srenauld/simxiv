extern crate proc_macro;
#[macro_use] extern crate syn;
extern crate csv;
#[macro_use] extern crate quote;

mod util;
mod xiv_csv;
mod embed_spelldata;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro]
pub fn embed_spelldata(input: TokenStream) -> TokenStream {
    let item:syn::LitStr = syn::parse(input).unwrap();
    embed_spelldata::derive_embed_spelldata(&item)
        .to_string()
        .parse()
        .unwrap()
}