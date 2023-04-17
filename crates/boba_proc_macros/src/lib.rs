use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Pearl)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);

    let (impl_gen, type_gen, where_gen) = generics.split_for_impl();
    let output = quote! {
        impl #impl_gen Pearl for #ident #type_gen #where_gen {}
    };
    output.into()
}
