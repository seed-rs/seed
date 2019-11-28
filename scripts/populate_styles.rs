//! ```cargo
//! [dependencies]
//! reqwest = "^0.9.20"
//! serde = { version = "^1.0.92", features = ["derive"] }
//! ```

//! Populate CSS styles

extern crate reqwest;
extern crate serde;

use serde::Deserialize;
use std::fs;

const STYLE_NAMES_FILE: &str = "./src/dom_entity_names/styles/style_names.rs";
const STYLES_ENDPOINT: &str = "https://seed-rs.github.io/html-css-db/css_properties.json";

#[derive(Debug, Deserialize)]
struct Style {
    name: StyleName
}

#[derive(Debug, Deserialize)]
struct StyleName {
    original: String,
    pascal_case: String,
}

fn main() {
    let styles = fetch_styles();
    let content_for_style_names_file = generate_content_for_style_names_file(styles);
    fs::write(STYLE_NAMES_FILE, content_for_style_names_file)
        .expect("Writing into style names file failed.")
}

/// Json example:
///
/// ```
/// [
///   {
///     "name": {
///       "original": "justify-content",
///       "pascal_case": "JustifyContent"
///     }
///   }
/// ]
/// ```
fn fetch_styles() -> Vec<Style> {
    reqwest::get(STYLES_ENDPOINT)
        .expect("Request to styles endpoints failed.")
        .json()
        .expect("Problem parsing CSS properties as JSON.")
}

/// Example of generated content:
///
/// ```rust,no_run
/// //! This file is generated automatically by `/scripts/populate_styles.rs`.
/// //! It's not meant to be edited directly.
///
/// make_styles! {
///   Display => "display",
///   JustifyContent => "justify-content",
/// }
/// ```
fn generate_content_for_style_names_file(styles: Vec<Style>) -> String {
    let style_pairs: String = styles
        .into_iter()
        .filter(|style| !style.name.pascal_case.is_empty())
        .map(|style| format!("    {} => \"{}\",\n", style.name.pascal_case, style.name.original))
        .collect();

    format!(r#"//! This file is generated automatically by `/scripts/populate_styles.rs`.
//! It's not meant to be edited directly.

make_styles! {{
    {}
}}
"#, style_pairs.trim().trim_end_matches(','))
}
