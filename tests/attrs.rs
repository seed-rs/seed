use seed::{prelude::*, *};

#[test]
fn test_attrs() {
    let attrs = attrs! {
        At::Class => "foo",
        At::Charset => String::from("utf8"),
        At::Hidden => true,
        At::Disabled => false,
        At::AutoFocus => None,
    };
    assert_eq!(attrs.vals.get(&At::Class), Some(&AtValue::from("foo")));
}
