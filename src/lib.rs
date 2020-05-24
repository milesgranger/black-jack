use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(DataFrame)]
#[allow(non_snake_case)]
pub fn DataFrame(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let dataframe_ident = format_ident!("{}{}", &ast.ident, "DataFrame");
    let dataframe_intoiterator_ident = format_ident!("{}IntoIterator", &dataframe_ident);
    let row_ident = &ast.ident;

    let data = match &ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("#[derive(DataFrame)] only supported for structs"),
    };
    let fields_named = match &data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("#[derive(DataFrame)] only supported for structs with named fields"),
    };
    let dataframe_fields = fields_named
        .named
        .iter()
        .map(|f: &Field| {
            let ty = &f.ty;
            let name = f.ident.as_ref().unwrap();
            quote! { #name: Vec<#ty> }
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
    let pub_fn_push = dataframe::push(&data, &row_ident);
    let pub_fn_select = dataframe::select(&row_ident, &field_names);
    let pub_fn_filter = dataframe::filter(&row_ident);
    let pub_fn_filter_inplace = dataframe::filter_inplace(&row_ident);
    let pub_fn_remove = dataframe::remove(&row_ident, &field_names);
    let pub_fn_is_empty = dataframe::is_empty();

    let attrs = &ast.attrs;

    (quote! {
        #(#attrs)*
        #[derive(Default)]
        pub struct #dataframe_ident {
            #(#dataframe_fields),*
        }
        impl #dataframe_ident {
            #pub_fn_new
            #pub_fn_len
            #pub_fn_push
            #pub_fn_select
            #pub_fn_filter
            #pub_fn_filter_inplace
            #pub_fn_remove
            #pub_fn_is_empty
            #(#attr_accessors)*
        }

        /// Implement DataFrame::into_iter()
        pub struct #dataframe_intoiterator_ident {
            df: #dataframe_ident
        }
        impl std::iter::IntoIterator for #dataframe_ident {
            type Item = #row_ident;
            type IntoIter = #dataframe_intoiterator_ident;

            fn into_iter(self) -> Self::IntoIter {
                #dataframe_intoiterator_ident { df: self }
            }
        }
        impl std::iter::Iterator for #dataframe_intoiterator_ident {
            type Item = #row_ident;

            fn next(&mut self) -> Option<Self::Item> {
                match self.df.is_empty() {
                    true => None,
                    false => Some(self.df.remove(0))
                }
            }
        }

        /// Implement DataFrame::FromIterator
        impl std::iter::FromIterator<#row_ident> for #dataframe_ident {
            fn from_iter<I: std::iter::IntoIterator<Item=#row_ident>>(iter: I) -> Self {
                let mut df = #dataframe_ident::new();
                for i in iter {
                    df.push(i);
                }
                df
            }
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

    pub fn is_empty() -> TokenStream {
        quote! {
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }
    }

    /// Generate `DataFrame::push(row)` method
    pub fn push(data: &DataStruct, row_ident: &Ident) -> TokenStream {
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
            pub fn push(&mut self, row: #row_ident) {
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
    /// Generate `DataFrame::filter_inplace(|row| ...)`
    pub fn filter_inplace(row_ident: &Ident) -> TokenStream {
        quote! {
            pub fn filter_inplace<F: Fn(&#row_ident) -> bool>(&mut self, condition: F) -> () {
                let mut n_removed = 0;
                for idx in 0..self.len() {
                    let row = self.select(idx - n_removed);
                    if condition(&row) == true {
                        let _ = self.remove(idx - n_removed);
                        n_removed += 1;
                    }
                }
            }
        }
    }

    pub fn remove(row_ident: &Ident, field_names: &[Ident]) -> TokenStream {
        let field_assignments = field_names
            .iter()
            .map(|ident| quote! { #ident: self.#ident.remove(idx) });
        quote! {
            pub fn remove(&mut self, idx: usize) -> #row_ident {
                #row_ident { #(#field_assignments),* }
            }
        }
    }
}
