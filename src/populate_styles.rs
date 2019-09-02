//! Populate CSS styles
//! HTML for viewing: https://www.w3.org/Style/CSS/all-properties.en.html
//! JSON for parsing: https://www.w3.org/Style/CSS/all-properties.en.json

//! MDN reference: https://developer.mozilla.org/en-US/docs/Web/CSS/Reference
//! Most common: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Properties_Reference
//! More: https://www.w3.org/Style/CSS/all-properties.en.html
//! https://developer.mozilla.org/en-US/docs/MDN/Contribute/Howto/Update_the_CSS_JSON_DB

use reqwest;
use serde::Deserialize;
use std::fs;

const STYLE_FILE: &'static str = "styles.rs";

/// See the Color Key section on [this page](https://www.w3.org/Style/CSS/all-properties.en.html)
#[derive(Debug, Deserialize)]
enum Status {
    Cr,  // Candidate recommendation
    Ed,  // Editor's draft
    Fpwd,  // First public working draft
    Lc, // Last call working draft
    Note,  // Working group note
    Pr, // Proposed recommendation
    Rec,  // Recommendation
    Wd,  // Working draft
}

impl Status {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_ref() {
            "cr" => Self::Cr,
            "ed" => Self::Ed,
            "fpwd" => Self::Fpwd,
            "lc" => Self::Lc,
            "note" => Self::Note,
            "pr" => Self::Pr,
            "rec" => Self::Rec,
            "wd" => Self::Wd,
            _ => panic!("Invalid CSS status")
        }
    }
}

#[derive(Debug, Deserialize)]
struct StyleData {
    property: String,
    url: String,
    status: String, // directly deserialize Status?
    title: String,
}

/// https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Convert a style property from its official name, to a Rust-like Enum variant name.
fn to_camel(s: &str) -> String {
    let mut result = String::new();
    for part in s.split('-') {
        result.push_str(&capitalize(part));
    }
    result
}


/// Create a rust file creates a `Style` enum, with variants of all valid CSS styles, in CamelCase.
fn make_file(styles: &[String]) {
    let mut text = r#"//! This file is generated automatically by populate_styles.rs.
It's not meant to be edited directly.

/// Similar to tag population.
macro_rules! make_styles {
    // Create shortcut macros for any element; populate these functions in this module.
    { $($st_camel:ident => $st:expr),+ } => {

        /// The St enum restricts element-creation to only valid styles.
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum St {
            $(
                $st_camel,
            )+
            Custom(String)
        }

        impl St {
            pub fn as_str(&self) -> &str {
                match self {
                    $ (
                        St::$st_camel => $st,
                    ) +
                    St::Custom(val) => &val
                }
            }
        }

        impl From<&str> for St {
            fn from(st: &str) -> Self {
                match st {
                    $ (
                          $st => St::$st_camel,
                    ) +
                    _ => {
                        crate::error(&format!("Can't find this attribute: {}", st));
                        St::Background
                    }
                }
            }
        }
        impl From<String> for St {
            fn from(st: String) -> Self {
                match st.as_ref() {
                    $ (
                          $st => St::$st_camel,
                    ) +
                    _ => {
                        crate::error(&format!("Can't find this attribute: {}", st));
                        St::Background
                    }
                }
            }
        }

    }
}

make_styles! {"#.to_string();

    for style in styles {
        text.push_str(&format!("{} => \"{}\",\n", to_camel(style), style));
    }
    text += "}\n";

    fs::write(STYLE_FILE, text);
}

/// Fetch style data, and store it in a Rust file.
pub fn main() {
    let data: Vec<StyleData> = reqwest::get("https://www.w3.org/Style/CSS/all-properties.en.html")
        .json().expect("Problem parsing CSS properties as JSON.");

    let styles: Vec<String> = data.iter().map(|d| d.property).collect();
    make_file(&styles);
}