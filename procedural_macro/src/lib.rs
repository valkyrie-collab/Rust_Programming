use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input, DeriveInput};

#[proc_macro_derive(MyDerive)]
pub fn my_personal_derive(item: TokenStream) -> TokenStream {
    let inputs: DeriveInput = parse_macro_input!(item as DeriveInput);

    let name: &Ident = &inputs.ident;

    let expand: proc_macro2::TokenStream = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "This is struct: {}", stringify!(#name))
            }
        }
    };

    expand.into()
}