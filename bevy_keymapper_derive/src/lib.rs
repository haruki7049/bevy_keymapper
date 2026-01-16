use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Environment)]
pub fn derive_environment(input: TokenStream) -> TokenStream {
    // Parse the input struct
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Generate the implementation
    let expanded = quote! {
        impl #impl_generics Environment for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}
