extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Meta};
use quote::{quote};

#[proc_macro_derive(VariableDescriptor, attributes(value))]
pub fn const_variable_descriptor_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_variable_macro(&input)
}

fn impl_variable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Extract the value attribute
    let value_attr = ast
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("value"))
        .and_then(|attr| attr.parse_meta().ok());

    let value = match value_attr {
        Some(Meta::NameValue(path)) => {
            path.lit
        }
        _ => panic!("Value attribute not found"),
    };

    let gen = quote!(
        impl VariableDescriptor for #name {
            fn output(&self, _t: f32) -> f32 {
                #value
            }
        }
    );

    gen.into()
}