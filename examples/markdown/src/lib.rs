use seed::{prelude::*, *};

// ----- ------
//    Init
// ----- -----

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model
}

// ----- ------
//    Model
// ----- -----

struct Model;

// ------ ------
//    Update
// ----- ------

#[allow(clippy::empty_enum)]
enum Msg {}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

// ------ ------
//     View
// ------ ------

fn view(_model: &Model) -> Node<Msg> {
    div![
        // The class required by GitHub styles. See `index.html`.
        C!["markdown-body"],
        from_md(md_header()),
        from_md(include_str!("../md/examples.md")),
        // `footer.html` is generated by `build.rs` during compilation.
        raw!(include_str!("../md/generated_html/footer.html")),
    ]
}

const fn md_header() -> &'static str {
    "# Markdown Example

Intended as a demo of using `md!` for markdown conversion.

And how to convert MD to HTML during compilation and include the result in the app code.

```bash
cargo make start
```

---

Open [127.0.0.1](//127.0.0.1:8000) in your browser."
}

fn from_md<M>(md: &str) -> Vec<Node<M>> {
    let options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(md, options);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);
    El::from_html(None, &html_text)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
