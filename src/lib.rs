#![deny(clippy::all)]
#![deny(clippy::pedantic)]

#![allow(dead_code)]

extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::quote;

#[proc_macro]
pub fn murmur64(input: TokenStream) -> TokenStream {
    let arg = syn::parse_macro_input!(input as syn::Lit);

    match arg {
        syn::Lit::Str(ref input_str) => {
            let hash = fasthash::murmur3::hash32(input_str.value());

            println!("Generated hash: '{}' => {}", input_str.value(), hash);

            let ret = quote! {
                #hash
            };
            ret.into()
        },
        _ => {
            panic!("This macro only supports string hashing")
        },
    }
}