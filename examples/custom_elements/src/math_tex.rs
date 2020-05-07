use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(expression: &str) -> Node<Ms> {
    custom![Tag::from("math-tex"), expression,]
}
