/// Common Namespaces
#[derive(Debug, Clone, PartialEq)]
pub enum Namespace {
    Html,
    Svg,
    MathMl,
    Xul,
    Xbl,
    Custom(String),
}

// https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS
impl Namespace {
    pub fn as_str(&self) -> &str {
        use Namespace::*;
        match self {
            Html => "http://www.w3.org/1999/xhtml",
            Svg => "http://www.w3.org/2000/svg",
            MathMl => "http://www.w3.org/1998/mathml",
            Xul => "http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul",
            Xbl => "http://www.mozilla.org/xbl",
            Custom(s) => s,
        }
    }
}

impl From<String> for Namespace {
    fn from(ns: String) -> Self {
        match ns.as_ref() {
            "http://www.w3.org/1999/xhtml" => Namespace::Html,
            "http://www.w3.org/2000/svg" => Namespace::Svg,
            "http://www.w3.org/1998/mathml" => Namespace::MathMl,
            "http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul" => Namespace::Xul,
            "http://www.mozilla.org/xbl" => Namespace::Xbl,
            _ => Namespace::Custom(ns),
        }
    }
}
