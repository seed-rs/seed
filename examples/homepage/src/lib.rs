//! The Seed homepage - hosting the guide, acting as an example

#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Clone)]
enum Page {
    Guide,
    Changelog
}

#[derive(Clone)]
struct Model {
    page: Page,
    guide_page: usize,  // Index of our guide sections.
//    guide_sections: Vec<El<Msg>>,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::Guide,
            guide_page: 0,
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
        "cursor" => "pointer";
        "font-size" => "1.2em"
    };

    div![ style!{"display" => "flex"; "justify-content" => "right"},
        ul![
            a![ &link_style, "Guide", simple_ev("click", Msg::ChangePage(Page::Guide)) ],
            a![ &link_style, "Changelog", simple_ev("click", Msg::ChangePage(Page::Changelog)) ],
            a![ &link_style, "Repo", attrs!{"href" => "https://github.com/David-OConnor/seed"} ],
            a![ &link_style, "Quickstart repo", attrs!{"href" => "https://github.com/David-OConnor/seed-quickstart"} ],
            a![ &link_style, "Crate", attrs!{"href" => "https://crates.io/crates/seed"} ],
            a![ &link_style, "API docs", attrs!{"href" => format!("https://docs.rs/seed/{}/seed/", version)} ]
        ]
    ]
}

fn guide(sections: Vec<(&str, El<Msg>)>, guide_page: usize) -> El<Msg> {
    let menu_items: Vec<El<Msg>> = sections.iter().map(|s| h6![ s.0 ]).collect();

    div![ style! {
        "display" => "grid";
        "grid-template-columns" => "300px auto";
        "grid-temlate-rows" => "auto"
    },
        div![ style!{"display" => "flex"}, menu_items ],
        div![ style!{"display" => "flex"}
        ]

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


fn view(model: Model) -> El<Msg> {
    let version = "0.1.2";
    let sections = vec![];
    let changelog_entries = vec![];

    div![ style!{
        // todo: How do we do areas?
        "display" => "grid";
        "grid-template-columns" => "auto";
        "grid-template-rows" => "100px auto 100px"
        },
            section![ style!{"grid-row" => "1 / 2"; "grid-column" => "1 / 2"},
                header(version)
            ],
            section![ style!{"grid-row" => "2 / 3"; "grid-column" => "1 / 2"},
                match model.page {
                    Page::Guide => guide(sections, model.guide_page),
                    Page::Changelog => changelog(changelog_entries),
                }
            ],
            section![ style!{"grid-row" => "3 / 4"; "grid-column" => "1 / 2"},
                footer()
            ]
    ]
}


#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main");
}