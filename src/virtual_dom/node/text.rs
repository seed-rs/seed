use std::borrow::Cow;
use std::fmt;

/// For representing text nodes.
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Text)
/// [`web_sys` reference](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Text.html)
#[derive(Clone, Debug)]
pub struct Text {
    pub text: Cow<'static, str>,
    pub node_ws: Option<web_sys::Node>,
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Text {
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            text: text.into(),
            node_ws: None,
        }
    }

    pub fn strip_ws_node(&mut self) {
        self.node_ws.take();
    }
}
