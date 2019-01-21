extern crate proc_macro;
use proc_macro::TokenStream;

use syn;

#[proc_macro]
pub fn update(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.ident;
    let abi = &input.abi;
}