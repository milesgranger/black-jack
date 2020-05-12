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
        _ => panic!("#[derive(DataFrame)] only supported for structs"),
    };
    let fields_named = match &data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("#[derive(DataFrame)] only supported for structs with named fields"),
    };
    let fields = fields_named
        .named
        .iter()
        .map(|f: &Field| {
            let ty = &f.ty;
            let name = f.ident.as_ref().unwrap();
            quote! { pub #name: Vec<#ty> }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    let field_names = fields_named
        .named
        .iter()
        .map(|f: &Field| f.ident.clone().unwrap())
        .collect::<Vec<Ident>>();

    // Generate attribute accessor methods
    // ie. attr 'foo' on row struct will have DataFrame::foo() and DataFrame::foo_mut()
    let attr_accessors = dataframe::attr_accessors(&fields_named);

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
            #(#attr_accessors)*
        }
    })
    .into()
}

mod dataframe {

    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use syn::{DataStruct, Fields, FieldsNamed, Ident};

    /// Generate `DataFrame::push(row)` method
    pub fn new() -> TokenStream {
        quote! {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }

    /// For every field/attr, generate accessors.
    /// ie. row struct w/ attr 'foo' gets: `DataFrame::foo()` & `DataFrame::foo_mut()`
    pub fn attr_accessors(fields: &FieldsNamed) -> impl Iterator<Item = TokenStream> + '_ {
        fields.named.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_name_mut = format_ident!("{}_{}", &field_name, "mut");
            let ty = &field.ty;
            quote! {
                pub fn #field_name(&self) -> &[#ty] {
                    &self.#field_name
                }
                pub fn #field_name_mut(&mut self) -> &mut[#ty] {
                    &mut self.#field_name
                }
            }
        })
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
