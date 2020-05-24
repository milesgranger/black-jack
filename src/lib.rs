use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(DataFrame)]
pub fn dataframe(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let row_ident = &ast.ident;

    let data = match &ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("#[derive(DataFrame)] only supported for structs"),
    };
    let fields_named = match &data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("#[derive(DataFrame)] only supported for structs with named fields"),
    };

    #[allow(unused_variables)]
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
    let pub_fn_len = dataframe::len();
    let pub_fn_push = dataframe::push(&row_ident);
    let pub_fn_filter = dataframe::filter(&row_ident);
    let pub_fn_filter_inplace = dataframe::filter_inplace(&row_ident);
    let pub_fn_remove = dataframe::remove(&row_ident);
    let pub_fn_is_empty = dataframe::is_empty();

    (quote! {

        impl DataFrame<#row_ident> {
            #pub_fn_new
            #pub_fn_len
            #pub_fn_push
            #pub_fn_filter
            #pub_fn_filter_inplace
            #pub_fn_remove
            #pub_fn_is_empty
            #(#attr_accessors)*
        }

        // DataFrame::into_iter()
        impl std::iter::IntoIterator for DataFrame<#row_ident> {
            type Item = #row_ident;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.values.into_iter()
            }
        }

        // DataFrame::from_iter
        impl std::iter::FromIterator<#row_ident> for DataFrame<#row_ident> {
            fn from_iter<I: std::iter::IntoIterator<Item=#row_ident>>(iter: I) -> Self {
                let mut df: DataFrame<#row_ident> = DataFrame::default();
                for i in iter {
                    df.push(i);
                }
                df
            }
        }

        /// From<>
        impl<D, R> From<D> for DataFrame<#row_ident>
            where D: std::iter::Iterator<Item = R>,
                  R: Into<#row_ident>
        {
            fn from(data: D) -> Self {
                let mut df = <DataFrame<#row_ident>>::new();
                for row in data {
                    df.push(row.into());
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
    use syn::{FieldsNamed, Ident};

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
                pub fn #field_name(&self) -> impl Iterator<Item = &#ty> {
                    self.values.iter().map(|row| &row.#field_name)
                }
                pub fn #field_name_mut(&mut self) -> impl Iterator<Item = &mut #ty> {
                    self.values.iter_mut().map(|row| &mut row.#field_name)
                }
            }
        })
    }

    pub fn is_empty() -> TokenStream {
        quote! {
            pub fn is_empty(&self) -> bool {
                self.values.len() == 0
            }
        }
    }

    /// Generate `DataFrame::push(row)` method
    pub fn push(row_ident: &Ident) -> TokenStream {
        quote! {
            pub fn push(&mut self, row: #row_ident) {
                self.values.push(row)
            }
        }
    }

    pub fn len() -> TokenStream {
        quote! {
            pub fn len(&self) -> usize {
                self.values.len()
            }
        }
    }

    /// Generate `DataFrame::filter(|row| ...)`
    pub fn filter(row_ident: &Ident) -> TokenStream {
        quote! {
            pub fn filter<F: Fn(&#row_ident) -> bool>(self, condition: F) -> Self {
                let mut df = Self::default();
                for row in self.values {
                    if condition(&row) {
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
                    let row = &self.values[idx - n_removed];
                    if condition(row) == true {
                        let _ = self.remove(idx - n_removed);
                        n_removed += 1;
                    }
                }
            }
        }
    }

    pub fn remove(row_ident: &Ident) -> TokenStream {
        quote! {
            pub fn remove(&mut self, idx: usize) -> #row_ident {
                self.values.remove(idx)
            }
        }
    }
}
