use pulldown_cmark::{html::push_html, Options, Parser};
use std::fs;

fn main() {
    let md_footer = fs::read_to_string("md/footer.md").expect("read footer.md");
    let parser = Parser::new_ext(&md_footer, Options::all());
    let mut html = String::new();
    push_html(&mut html, parser);
    fs::write("md/generated_html/footer.html", html).expect("write footer.html");
}
