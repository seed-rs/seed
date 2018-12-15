//! The Seed homepage - hosting the guide, and acting as an example. Contains
//! simple interactions, markdown elements, and lots of view markup.

mod book;

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
struct GuideSection {
    title: String,
    element: El<Msg>
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
            let mut element = El::from_markdown(&md_text);
            guide_sections.push(GuideSection{title: title.to_string(), element});
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
    let menu_item_style = style!{
        "margin" => "auto";
        "cursor" => "pointer";
    };
    let menu_items: Vec<El<Msg>> = sections
        .iter()
        .enumerate()
        .map(|(i, s)|
        h3![ &menu_item_style, simple_ev("click", Msg::ChangeGuidePage(i)), s.title ]
    ).collect();

    div![ style! {
        "display" => "grid";
        "grid-template-columns" => "200px auto";
//        "grid-template-rows" => "1fr";
        "color" => "black";
        "grid-auto-rows" => "1fr";
        "align-items" => "start";
//        "padding" => 20;
    },
        div![ style!{"display" => "flex"; "flex-direction" => "column";
                     "grid-column" => "1 / 2";
//                      "grid-row" => "1 / 2";
                      "justify-content" => "flex-start";
                     "background-color" => "#bc4639"; "padding" => 20;},
            menu_items
        ],

        div![ style!{"display" => "flex"; "grid-column" => "2 / 3";
//                     "grid-row" => "1 / 2";
                     "padding" => 40; "background-color" => "#d4a59a";},
            sections[guide_page].clone().element
        ]
    ]
}

fn changelog_entry(version: &str, changes: Vec<&str>) -> El<Msg> {
    let changes: Vec<El<Msg>> = changes.iter().map(|c| li![ c ]).collect();
    div![
        h2![ version ],
        ul![
            changes
        ]
    ]
}

fn changelog(entries: Vec<El<Msg>>) -> El<Msg> {
    div![ style!{
            "display" => "flex";
            "flex-direction" => "column";
            "align-items" => "center";
            "background-color" => "#d4a59a";
            "padding" => 50;
            "color" => "black";
    },
        entries
    ]
}

fn footer() -> El<Msg> {
    div![ style!{"display" => "flex"; "justify-content" => "center"},
        h4![ "© 2019 David O'Connor"]
    ]
}



fn view(model: Model) -> El<Msg> {
    let version = "0.1.4";
    let changelog_entries = vec![
        changelog_entry("v0.1.0", vec![ "Initial release" ]),
    ];

    div![
//        style!{
//            // todo: How do we do areas?
//            "display" => "grid";
//            "grid-template-columns" => "auto";
//            "grid-template-rows" => "100px auto auto 100px"
//        },
        style!{
            // todo: How do we do areas?
            "display" => "flex";
            "flex-direction" => "column";
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