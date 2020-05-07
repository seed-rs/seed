use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(lang: &str, code: &str) -> Node<Ms> {
    custom![
        Tag::from("code-block"),
        attrs! {
            At::from("lang") => lang,
            At::from("code") => code,
        }
    ]
}
