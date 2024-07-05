extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(WrappedValue)]
pub fn wrapped_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl WrappedValue for #name {
            fn get(&self) -> f32 {
                self.0
            }

            fn set(&mut self, value: f32) {
                self.0 = value;
            }
        }
    };

    TokenStream::from(expanded)
}
