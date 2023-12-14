use proc_macro::TokenStream;
use quote::quote;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use syn::{parse::Parse, parse_macro_input, LitStr};

struct Args {
    path: Option<LitStr>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = if input.is_empty() {
            None
        } else {
            Some(input.parse()?)
        };
        Ok(Args { path })
    }
}

#[proc_macro]
pub fn i18n(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Args);
    let j = if let Some(path) = input.path {
        path.value()
    } else {
        "locales".to_string()
    };
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push::<PathBuf>(j.into());
    let path = current_dir.canonicalize().unwrap();
    let mut jmap = HashMap::new();
    let x = fs::read_dir(path).unwrap();
    x.fold(&mut jmap, |acc, x| {
        let x = x.unwrap();
        let path = x.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let path = path.to_str().unwrap();
            let dir = fs::read_dir(path).unwrap();
            let second_map2 = dir.fold(HashMap::new(), |mut acc, x| {
                let x = x.unwrap();
                let path = x.path();
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let path = path.to_str().unwrap();
                let content = fs::read_to_string(path).unwrap();
                let json_content: Value = serde_json::from_str(&content).unwrap();
                acc.insert(name, json_content);
                acc
            });
            acc.insert(name.to_string(), second_map2);
        }
        acc
    });
    println!("{:?}", jmap);
    generate_code().into()
}

fn generate_code() -> proc_macro2::TokenStream {
    quote! {

        pub fn i18n_content() {
            println!("i18n_content");
        }
    }
}
