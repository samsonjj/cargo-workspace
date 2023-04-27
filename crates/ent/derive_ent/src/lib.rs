use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

mod derive_fields;

type Ast = syn::ItemStruct;

fn parse(tokens: proc_macro2::TokenStream) -> Ast {
    let ast: ItemStruct = syn::parse2(tokens.into()).unwrap();
    ast
}

#[derive(Debug, PartialEq)]
enum ModelFieldType {
    Int,
    String,
}

#[derive(Debug, PartialEq)]
struct Model {
    fields: Vec<ModelField>,
}

#[derive(Debug, PartialEq)]
struct ModelField {
    ident_rust: String,
    ty: ModelFieldType,
}

fn analyze(ast: Ast) -> Model {
    let mut model = Model { fields: vec![] };

    // collect fields
    model.fields = ast
        .fields
        .iter()
        .map(|f| {
            let ident_rust = f.ident.as_ref().unwrap().to_string();
            let ty = match &f.ty {
                syn::Type::Path(tp) => {
                    assert!(
                        tp.path.segments.len() == 1,
                        "type path segments has a length that is not 1"
                    );
                    println!("hello");
                    match tp.path.segments[0].ident.to_string().as_ref() {
                        "i32" => ModelFieldType::Int,
                        "String" => ModelFieldType::String,
                        id => panic!("Unsupported field type {:?}", id),
                    }
                }
                x => panic!("Unsupported field type {:?}", quote!(#x)),
            };
            ModelField { ident_rust, ty }
        })
        .collect();

    model
}

#[proc_macro_derive(Ent, attributes(primary_key))]
pub fn derive_ent(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: Ast = parse(tokens.into());
    let model = analyze(ast);
    TokenStream::new().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = quote!(
            struct Point {
                x: i32,
                y: i32,
            }
        );
    }

    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/*.rs");
    }

    #[test]
    fn test_analyze() {
        let tokens = quote!(
            struct Foo {
                bar: i32,
                message: String,
            }
        );

        let model = analyze(parse(tokens));

        assert_eq!(
            model,
            Model {
                fields: vec![
                    ModelField {
                        ident_rust: "bar".to_string(),
                        ty: ModelFieldType::Int,
                    },
                    ModelField {
                        ident_rust: "message".to_string(),
                        ty: ModelFieldType::String,
                    }
                ]
            }
        );
    }
}
