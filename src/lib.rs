use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(DataFrame)]
#[allow(non_snake_case)]
pub fn DataFrame(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let dataframe_name = format_ident!("{}{}", &ast.ident, "DataFrame");
    let struct_name = &ast.ident;

    let data = match &ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("derive(DataFrame) only supported for structs"),
    };
    let fields = match &data.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f: &Field| {
                let ty = &f.ty;
                let name = f.ident.as_ref().unwrap();
                quote! { pub #name: Vec<#ty> }
            })
            .collect::<Vec<proc_macro2::TokenStream>>(),
        _ => vec![],
    };
    let field_names = match &data.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f: &Field| f.ident.clone().unwrap())
            .collect::<Vec<Ident>>(),
        _ => vec![],
    };

    // generate methods
    let pub_fn_new = dataframe::new();
    let pub_fn_len = dataframe::len(&field_names);
    let pub_fn_push = dataframe::push(&data, &struct_name);
    let pub_fn_select = dataframe::select(&struct_name, &field_names);
    let pub_fn_filter = dataframe::filter(&struct_name);

    let attrs = &ast.attrs;
    (quote! {
        #(#attrs)*
        #[derive(Default)]
        pub struct #dataframe_name {
            #(#fields),*
        }
        impl #dataframe_name {
            #pub_fn_new
            #pub_fn_len
            #pub_fn_push
            #pub_fn_select
            #pub_fn_filter
        }
    })
    .into()
}

mod dataframe {

    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{DataStruct, Fields, Ident};

    /// Generate `DataFrame::push(row)` method
    pub fn new() -> TokenStream {
        quote! {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }

    /// Generate `DataFrame::push(row)` method
    pub fn push(data: &DataStruct, struct_name: &Ident) -> TokenStream {
        let push_code = if let Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .map(|ident| quote! { self.#ident.push(row.#ident); })
        } else {
            panic!("Expected named fields")
        };

        quote! {
            pub fn push(&mut self, row: #struct_name) {
                #(#push_code)*
            }
        }
    }

    pub fn len(field_names: &[Ident]) -> TokenStream {
        let field = &field_names[0];
        quote! {
            pub fn len(&self) -> usize {
                self.#field.len()
            }
        }
    }

    pub fn select(row_ident: &Ident, field_names: &[Ident]) -> TokenStream {
        let field_assignments = field_names
            .iter()
            .map(|ident| quote! { #ident: self.#ident[idx].clone() });
        quote! {
            pub fn select(&self, idx: usize) -> #row_ident {
                #row_ident { #(#field_assignments),* }
            }
        }
    }

    /// Generate `DataFrame::filter(|row| ...)`
    pub fn filter(row_ident: &Ident) -> TokenStream {
        quote! {
            pub fn filter<F: Fn(&#row_ident) -> bool>(&self, condition: F) -> Self {
                let mut df = Self::default();
                for idx in 0..=df.len() {
                    let row = self.select(idx);
                    if condition(&row) == true {
                        df.push(row);
                    }
                }
                df
            }
        }
    }
}
