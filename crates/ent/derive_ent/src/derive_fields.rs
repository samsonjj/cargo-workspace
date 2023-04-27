use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn f_derive_fields(ast: ItemStruct) -> TokenStream {
    let struct_ident = ast.ident.to_string().to_case(Case::Camel);
    let fields = ast
        .fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string().to_case(Case::Camel));

    quote! {
        pub enum #struct_ident {
            #(#fields),*
        }
    }
}
