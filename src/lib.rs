use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, Fields};

#[proc_macro_derive(DataFrame)]
#[allow(non_snake_case)]
pub fn DataFrame(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let new_name = format_ident!("{}{}", &ast.ident, "DataFrame");

    let data = match &ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("derive(DataFrame) only supported for structs"),
    };
    let fields = match &data.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(Clone::clone)
            .map(|f: Field| {
                let ty = f.ty;
                let name = format_ident!("{}", f.ident.unwrap().to_string());
                let ts = quote! { pub #name: Vec<#ty> };
                ts
            })
            .collect::<Vec<proc_macro2::TokenStream>>(),
        _ => vec![],
    };

    let field_names = match &data.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(Clone::clone)
            .map(|f: Field| {
                let name = format_ident!("{}", f.ident.unwrap().to_string());
                name
            })
            .collect::<Vec<syn::Ident>>(),
        _ => vec![],
    };

    let name = &ast.ident;

    let push_code = field_names.iter().map(|ident| {
        quote! { self.#ident.push(row.#ident); }
    });

    (quote! {
        #[derive(Default)]
        pub struct #new_name {
            #(#fields),*
        }
        impl #new_name {
            pub fn new() -> Self {
                Self::default()
            }
            pub fn push(&mut self, row: #name) {
                #(#push_code)*
            }
        }
    })
    .into()
}
