use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse::Parse, LitStr};

struct Args{
    path: Option<LitStr>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = if input.is_empty() {
            None
        } else {
            Some(input.parse()?)
        };
        Ok(Args{ path })
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
    let f: std::path::PathBuf = j.into();
    current_dir.push(&f);
    println!("current dir: {} {:?}", current_dir.canonicalize().unwrap().display(), f);
    generate_code().into()
}

fn generate_code() -> proc_macro2::TokenStream {
    quote! {
        use std::path::PathBuf;

        pub fn i18n_content() {
            println!("i18n_content");
        }
    }
}
