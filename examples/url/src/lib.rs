use seed::{prelude::*, *};
use std::rc::Rc;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model::new(url, orders.clone_base_path())
}

// ------ ------
//     Model
// ------ ------

struct Model {
    base_path: Rc<Vec<String>>,
    initial_url: Url,
    next_path_part: Option<String>,
    remaining_path_parts: Vec<String>,
    base_url: Url,
}

impl Model {
    fn new(mut url: Url, base_path: Rc<Vec<String>>) -> Self {
        log!(&url);
        log!(url.to_string());
        log!("_______________________________");

        Self {
            base_path,
            initial_url: url.clone(),
            base_url: url.clone().truncate_relative_path(),
            next_path_part: url.pop_relative_path_part().map(ToOwned::to_owned),
            remaining_path_parts: url
                .consume_relative_path()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        }
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
    GoToUrl(Url),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            *model = Model::new(url, orders.clone_base_path())
        }
        Msg::GoToUrl(url) => {
            orders.notify(subs::UrlRequested::new(url));
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    ol![
        li![
            button![
                "Go to '/ui/a/b/c?x=1?#hash'` and reload the page",
                ev(Ev::Click, |_| {
                    Url::new()
                        .set_path(&["ui", "a", "b", "c"])
                        .set_search(vec![
                            ("x", vec!["1"])
                        ].into_iter().collect())
                        .set_hash("hash")
                        .go_and_load()
                })
            ],
        ],
        li![
            format!("Base path ...... \"{}\"  ......  (comment out `base` element in `index.html`, refresh the page and watch changes)", &model.base_path.join("/")),
        ],
        li![
            format!("Initial Url ...... \"{}\"", &model.initial_url),
        ],
        li![
            format!("Base Url ...... \"{}\"  ......  (the path part is the most important here)", &model.base_url),
        ],
        li![
            format!("Next path part ...... \"{:?}\"", &model.next_path_part),
        ],
        li![
            format!("Remaining path parts ...... \"{:?}\"", &model.remaining_path_parts),
        ],
        li![
            button![
                "Go to '/' and don't trigger `UrlChanged`",
                ev(Ev::Click, |_| {
                    Url::new().go_and_push()
                })
            ],
        ],
        li![
            button![
                "Go back",
                ev(Ev::Click, |_| {
                    Url::go_back(1)
                })
            ],
        ],
        li![
            button![
                "Go to '/' and trigger `UrlChanged` (simulate `<a>` link click)",
                ev(Ev::Click, |_| Msg::GoToUrl(Url::new()))
            ],
        ],
        li![
            button![
                "Go to 'https://example.com'",
                ev(Ev::Click, |_| {
                    Url::go_and_load_with_str("https://example.com")
                })
            ],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
