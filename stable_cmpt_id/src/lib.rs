use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};
#[proc_macro_derive(StableID)]
pub fn derive_stable_type_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name_str = name.to_string();

    let expanded = quote! {
        impl #impl_generics crate::StableTypeID for #name #ty_generics #where_clause {
            const ID: u64 = {
                let path = concat!(module_path!(), "::", #name_str);

                let bytes = path.as_bytes();
                let mut hash = 0xcbf29ce484222325u64;
                let mut i = 0;
                while i < bytes.len() {
                    hash ^= bytes[i] as u64;
                    hash = hash.wrapping_mul(0x100000001b3u64);
                    i += 1;
                }
                hash
            };
        }
    };

    TokenStream::from(expanded)
}
