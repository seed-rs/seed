//! The Seed homepage - hosting the guide, and acting as an example. Contains
//! simple interactions, and lots of view markup.

mod book;

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;

use pulldown_cmark;

// Model

#[derive(Clone)]
enum Page {
    Guide,
    Changelog
}

#[derive(Clone)]
struct GuideSection {
    title: String,
//    element: El<Msg>
    html: String,
}

#[derive(Clone)]
struct Model {
    page: Page,
    guide_page: usize,  // Index of our guide sections.
    guide_sections: Vec<GuideSection>,
}


// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        let mut guide_sections = Vec::new();
        let md_texts = vec![
            ("First", crate::book::guide::text()),
        ];

        for (title, md_text) in md_texts {
            let parser = pulldown_cmark::Parser::new(&md_text);
            let mut html_buf = String::new();
            pulldown_cmark::html::push_html(&mut html_buf, parser);
//            let mut element = div![];
//            element.el_ws;

            guide_sections.push(GuideSection{title: title.to_string(), html: html_buf});
        }

        Self {
            page: Page::Guide,
            guide_page: 0,
            guide_sections,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    ChangePage(Page),
    ChangeGuidePage(usize),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::ChangePage(page) => Model {page, ..model},
        Msg::ChangeGuidePage(guide_page) => Model {guide_page, ..model},
    }
}


// View


fn header(version: &str) -> El<Msg> {
    let link_style = style!{
        "margin-left" => 20;
        "margin-right" => 20;
        "font-weight" => "bold";
        "font-size" => "1.2em";
//        "text-decoration" => "none";
    };

    div![ style!{"display" => "flex"; "justify-content" => "right"},
        ul![
            a![ &link_style, "Guide", attrs!{"href" => "#/guide"}, simple_ev("click", Msg::ChangePage(Page::Guide)) ],
            a![ &link_style, "Changelog", attrs!{"href" => "#/changelog"}, simple_ev("click", Msg::ChangePage(Page::Changelog)) ],
            a![ &link_style, "Repo", attrs!{"href" => "https://github.com/David-OConnor/seed"} ],
            a![ &link_style, "Quickstart repo", attrs!{"href" => "https://github.com/David-OConnor/seed-quickstart"} ],
            a![ &link_style, "Crate", attrs!{"href" => "https://crates.io/crates/seed"} ],
            a![ &link_style, "API docs", attrs!{"href" => format!("https://docs.rs/seed/{}/seed/", version)} ]
        ]
    ]
}

fn title() -> El<Msg> {
    div![ style!{
            "display" => "flex";
            "flex-direction" => "column";
            "align-items" => "center";
            },
        h1![ style!{"font-size" => "2em"}, "Seed" ],
        p![ style!{"font-size" => "1.5em"}, "A tool for building interactive webapps with Rust" ],
    ]
}

fn guide(sections: Vec<GuideSection>, guide_page: usize) -> El<Msg> {
    let menu_style = style!{
        "margin" => "auto";
        "cursor" => "pointer";
    };
    let menu_items: Vec<El<Msg>> = sections
        .iter()
        .enumerate()
        .map(|(i, s)|
        h3![ &menu_style, simple_ev("click", Msg::ChangeGuidePage(i)), s.title ]
    ).collect();

    // We manually set the inner_html here vice in init, since it may be rewritten
    // by events.
    let mut markdown_element = div![];
    let el_websys = markdown_element.el_ws.take().unwrap();
//    el_websys.set_inner_html(&sections[guide_page].clone().html);

    markdown_element.el_ws.replace(el_websys);

    div![ style! {
        "display" => "grid";
        "grid-template-columns" => "300px auto";
        "grid-temlate-rows" => "auto";
        "background-color" => "#d4a59a";
        "color" => "black";
    },
        div![ style!{"display" => "flex"},
            menu_items
        ],

        markdown_element,
//        div![ style!{"display" => "flex"},
//            sections[guide_page].clone().html
//        ]

    ]
}

fn changelog(entries: Vec<El<Msg>>) -> El<Msg> {
    div![ style!{"display" => "flex"}, entries ]
}

fn footer() -> El<Msg> {
    div![ style!{"display" => "flex"; "justify-content" => "center"},
        h4![ "© 2019 David O'Connor"]
    ]
}




fn quickstart() -> El<Msg> {
    div![
        h2![ "Quickstart" ],

        h3![ "Setup" ],
        p![ " This framework requires you to install [Rust](https://www.rust-lang.org/tools/install) - This will
enable the CLI commands below:

 You'll need a recent version of Rust: `rustup update`

The wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`

And wasm-bindgen: `cargo install wasm-bindgen-cli`"
        ],
        h3![ "The theoretical minium"],

        span![
"
## Test

```
log!(TEST)
```

"

        ]

    ]
}


fn view(model: Model) -> El<Msg> {
    let version = "0.1.4";
    let changelog_entries = vec![];

    div![ style!{
        // todo: How do we do areas?
        "display" => "grid";
        "grid-template-columns" => "auto";
        "grid-template-rows" => "100px auto auto 100px"
    },
        section![ style!{"grid-row" => "1 / 2"; "grid-column" => "1 / 2"},
            header(version)
        ],
        section![ style!{"grid-row" => "2 / 3"; "grid-column" => "1 / 2"},
            title()
        ],
        section![ style!{"grid-row" => "3 / 4"; "grid-column" => "1 / 2"},
            match model.page {
                Page::Guide => guide(model.guide_sections, model.guide_page),
                Page::Changelog => changelog(changelog_entries),
            }
        ],
        section![ style!{"grid-row" => "4 / 5"; "grid-column" => "1 / 2"},
            footer()
        ]
    ]
}


#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main");
}