/// Allows to use different types of input values in `C!` macro.
pub trait ToClasses {
    fn to_classes(self) -> Option<Vec<String>>;
}

// ------ Implementations ------

impl<T: ToClasses + Clone> ToClasses for &T {
    fn to_classes(self) -> Option<Vec<String>> {
        self.clone().to_classes()
    }
}

// --- Texts ---

impl ToClasses for String {
    fn to_classes(self) -> Option<Vec<String>> {
        Some(vec![self])
    }
}

impl ToClasses for &str {
    fn to_classes(self) -> Option<Vec<String>> {
        Some(vec![self.to_string()])
    }
}

// --- Containers ---

impl<T: ToClasses> ToClasses for Option<T> {
    fn to_classes(self) -> Option<Vec<String>> {
        self.and_then(ToClasses::to_classes)
    }
}

impl<T: ToClasses> ToClasses for Vec<T> {
    fn to_classes(self) -> Option<Vec<String>> {
        let classes = self.into_iter().filter_map(ToClasses::to_classes).flatten();
        Some(classes.collect())
    }
}

impl<T: ToClasses + Clone> ToClasses for &[T] {
    fn to_classes(self) -> Option<Vec<String>> {
        let classes = self.iter().filter_map(ToClasses::to_classes).flatten();
        Some(classes.collect())
    }
}

// ------ ------ Tests ------ ------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::virtual_dom::AtValue;
    use wasm_bindgen_test::*;

    // --- Texts ---

    #[wasm_bindgen_test]
    fn to_classes_ref_str() {
        let text: &str = "foo";
        assert_eq!(C![text].vals[&At::Class], AtValue::Some("foo".to_owned()));
    }

    #[wasm_bindgen_test]
    fn to_classes_ref_str_empty() {
        let text: &str = "";
        assert!(C![text].vals.get(&At::Class).is_none());
    }

    #[wasm_bindgen_test]
    fn to_classes_string() {
        let text: String = String::from("bar");
        assert_eq!(C![text].vals[&At::Class], AtValue::Some("bar".to_owned()));
    }

    #[wasm_bindgen_test]
    fn to_classes_ref_string() {
        let text: &String = &String::from("ref_bar");
        assert_eq!(
            C![text].vals[&At::Class],
            AtValue::Some("ref_bar".to_owned())
        );
    }

    // --- Containers ---

    #[wasm_bindgen_test]
    fn to_classes_vec() {
        let vec: Vec<&str> = vec!["foo_1", "foo_2"];
        assert_eq!(
            C![vec].vals[&At::Class],
            AtValue::Some("foo_1 foo_2".to_owned())
        );
    }

    #[wasm_bindgen_test]
    fn to_classes_ref_vec() {
        let vec: &Vec<&str> = &vec!["foo_1", "foo_2"];
        assert_eq!(
            C![vec].vals[&At::Class],
            AtValue::Some("foo_1 foo_2".to_owned())
        );
    }

    #[wasm_bindgen_test]
    fn to_classes_slice() {
        let slice: &[&str] = &["foo_1", "foo_2"];
        assert_eq!(
            C![slice].vals[&At::Class],
            AtValue::Some("foo_1 foo_2".to_owned())
        );
    }

    #[wasm_bindgen_test]
    fn to_classes_option_some() {
        let option: Option<&str> = Some("foo_opt");
        assert_eq!(
            C![option].vals[&At::Class],
            AtValue::Some("foo_opt".to_owned())
        );
    }

    #[wasm_bindgen_test]
    fn to_classes_ref_option_some() {
        let option: &Option<&str> = &Some("foo_opt");
        assert_eq!(
            C![option].vals[&At::Class],
            AtValue::Some("foo_opt".to_owned())
        );
    }

    #[wasm_bindgen_test]
    fn to_classes_option_none() {
        let option: Option<&str> = None;
        assert!(C![option].vals.get(&At::Class).is_none());
    }

    #[wasm_bindgen_test]
    fn to_classes_option_vec() {
        let option_vec: Option<Vec<&str>> = Some(vec!["foo_1", "foo_2"]);
        assert_eq!(
            C![option_vec].vals[&At::Class],
            AtValue::Some("foo_1 foo_2".to_owned())
        );
    }
}
