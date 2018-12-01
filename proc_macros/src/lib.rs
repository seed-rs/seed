//#![feature(proc_macro_hygiene)]
//#![feature(proc_macro_quote)]
//#![feature(proc_macro_span)]
//#![feature(proc_macro_diagnostic)]
//#![feature(proc_macro_raw_ident)]

extern crate proc_macro;
extern crate syn;
extern crate quote;
//extern crate framework;

//use proc_macro::{TokenStream};  // todo why nto?


#[proc_macro]
pub fn div(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

//    quote!(framework::dom_types::El::new());

//    let parsed = parse_macro_input!(input as HtmlRoot);
//    parsed.expand().into()
//    let mut vec = Vec::new();

//    let el = temp::El::new(temp::Tag::Div);
//
////    panic!("{:#?}", parse_macro_input!(item as Item));
//
//    for tt in input.clone() {
//        match tt {
////            proc_macro::TokenTree::Ident(ident) => {tokens.push(ident)},
////            _ => {println!("Other")},
//        }
//    }

//    el.into_token_stream()


//        const dummy_const: () = {
//            extern crate framework = { path = "../"}
//        };


//    let dummy = syn::parse_macro_input!(input);

    input
}



