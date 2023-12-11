use proc_macro::TokenStream;

#[proc_macro]
pub fn i18n(_item: TokenStream) -> TokenStream {
    "fn i18n() -> u32 { 42 }".parse().unwrap()
}
