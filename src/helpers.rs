use std::ops::Not;

/// Alternative to `!`.
///
/// # Example
///
/// ```rust,no_run
///div![
///    "Button",
///    IF!(not(disabled) => ev(Ev::Click, Msg::Clicked)),
///]
/// ```
pub fn not<T: Not>(predicate: T) -> T::Output {
    predicate.not()
}

// @TODO move helpers from lib.rs or shortcuts.rs here

// ------ ------ Tests ------ ------

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn helpers_not() {
        assert!(not(false));
    }
}
