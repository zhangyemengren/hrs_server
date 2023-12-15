mod i18n;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn i18n(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as i18n::Args);
    let data = i18n::generate_data(input);
    i18n::generate_code(data).into()
}
