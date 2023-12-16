use quote::quote;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use syn::{parse::Parse, LitStr};

pub struct Args {
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

pub type I18nMap = HashMap<String, HashMap<String, Value>>;

pub fn generate_data(input: Args) -> I18nMap {
    let target = if let Some(path) = input.path {
        path.value()
    } else {
        "locales".to_string()
    };
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push::<PathBuf>(target.into());
    let path = current_dir.canonicalize().unwrap();
    let mut jmap = HashMap::new();
    let path_dir = fs::read_dir(path).unwrap();
    read_dir(path_dir, &mut jmap);
    jmap
}

fn read_dir(x: ReadDir, jmap: &mut I18nMap) {
    x.for_each(|x| {
        let x = x.unwrap();
        let path = x.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let path = path.to_str().unwrap();
            let dir = fs::read_dir(path).unwrap();
            let second_map2 = read_sec_dir_insert_json(dir);
            jmap.insert(name.to_string(), second_map2);
        }
    });
}

fn read_sec_dir_insert_json(dir: ReadDir) -> HashMap<String, Value> {
    let mut acc = HashMap::new();
    dir.for_each(|x| {
        let x = x.unwrap();
        let path = x.path();
        if path.is_dir() {
            return;
        }
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        let path = path.to_str().unwrap();
        let content = fs::read_to_string(path).unwrap();
        let json_content: Value = serde_json::from_str(&content).unwrap();
        acc.insert(name, json_content);
    });
    acc
}

pub fn generate_code(data: I18nMap) -> proc_macro2::TokenStream {
    let mut entries = Vec::new();

    data.iter().for_each(|(lang, translations)| {
        let mut trans_entries = Vec::new();

        translations.iter().for_each(|(key, value)| {
            let value_str = serde_json::to_string(&value).unwrap_or_default();
            trans_entries.push(quote! { map.insert(#key.to_string(), #value_str.to_string()); });
        });

        entries.push(quote! {
            let mut map = HashMap::new();
            #(#trans_entries)*
            outer_map.insert(#lang.to_string(), map);
        });
    });

    let x = quote! {
        let mut outer_map = HashMap::new();
        #(#entries)*
        outer_map
    };

    quote! {
        use once_cell::sync::Lazy;
        use std::collections::HashMap;
        use serde_json::Value;

        static I18N_MAP: Lazy<HashMap<String, HashMap<String, String>>> = Lazy::new(|| {
            #x.into()
        });
    }
}
