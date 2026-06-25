#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn event(attribute: TokenStream, item: TokenStream) -> TokenStream {
    if !attribute.is_empty() {
        return quote! {
            compile_error!("#[event] does not accept arguments");
        }
        .into();
    }

    let input = parse_macro_input!(item as ItemStruct);

    let struct_visibility = &input.vis;
    let struct_name = &input.ident;
    let generics = &input.generics;

    let named_fields = match &input.fields {
        Fields::Named(fields) => fields,
        Fields::Unnamed(_) => {
            return quote! {
                compile_error!("#[event] only supports structs with named fields");
            }
            .into();
        }
        Fields::Unit => {
            return quote! {
                compile_error!("#[event] only supports structs with named fields");
            }
            .into();
        }
    };

    let constructor_args = named_fields.named.iter().map(|field| {
        let field_name = field
            .ident
            .as_ref()
            .expect("named field should always have an identifier");

        let field_type = &field.ty;

        quote! {
            #field_name: #field_type
        }
    });

    let constructor_fields = named_fields.named.iter().map(|field| {
        let field_name = field
            .ident
            .as_ref()
            .expect("named field should always have an identifier");

        quote! {
            #field_name
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #input

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #struct_visibility fn new(#(#constructor_args),*) -> Self {
                Self {
                    #(#constructor_fields),*
                }
            }
        }
    }
    .into()
}
