use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(icon: &str, width: Option<u32>, height: Option<u32>) -> Node<Ms> {
    custom![
        Tag::from("feather-icon"),
        attrs!{
            At::from("icon") => icon,
            At::Width => if let Some(width) = width { AtValue::Some(width.to_string()) } else { AtValue::Ignored },
            At::Height => if let Some(height) = height { AtValue::Some(height.to_string()) } else { AtValue::Ignored },
        }
    ]
}