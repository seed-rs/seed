use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(icon: &str, width: Option<u32>, height: Option<u32>) -> Node<Ms> {
    custom![
        Tag::from("feather-icon"),
        attrs! {
            At::from("icon") => icon,
            At::Width => width.map_or(AtValue::Ignored, |width| AtValue::Some(width.to_string())),
            At::Height => height.map_or(AtValue::Ignored, |height| AtValue::Some(height.to_string())),
        }
    ]
}
