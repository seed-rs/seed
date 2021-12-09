use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use seed::{prelude::*, *};

const STORAGE_KEY: &str = "seed_unsaved_changes_text";

// ------ ------
//     Init
// ------ ------

#[allow(clippy::needless_pass_by_value)]
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // https://developer.mozilla.org/en-US/docs/Web/API/WindowEventHandlers/onbeforeunload
    orders
        .stream(streams::window_event(Ev::BeforeUnload, Msg::BeforeUnload))
        .subscribe(Msg::UrlRequested);

    let text = LocalStorage::get(STORAGE_KEY).unwrap_or_default();
    Model {
        base_url: url.to_base_url(),
        saved_text_hash: calculate_hash(&text),
        text,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    base_url: Url,
    saved_text_hash: u64,
    text: String,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn no_home(self) -> Url {
        self.base_url().add_path_part("no-home")
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    TextChanged(String),
    Save,
    UrlRequested(subs::UrlRequested),
    BeforeUnload(web_sys::Event),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::TextChanged(text) => model.text = text,
        Msg::Save => {
            LocalStorage::insert(STORAGE_KEY, &model.text).expect("save text");
            model.saved_text_hash = calculate_hash(&model.text);
        }
        Msg::UrlRequested(subs::UrlRequested(_, url_request)) => {
            if calculate_hash(&model.text) == model.saved_text_hash {
                return;
            }
            if Ok(true)
                == window().confirm_with_message("Do you want to leave? Data won't be saved.")
            {
                return;
            }
            url_request.handled_and_prevent_refresh();
        }
        Msg::BeforeUnload(event) => {
            if calculate_hash(&model.text) != model.saved_text_hash {
                log!("attempt to prevent navigation");
                let event = event.unchecked_into::<web_sys::BeforeUnloadEvent>();
                event.prevent_default();
                // Because of Chrome
                event.set_return_value("");
            }
        }
    }
}

fn calculate_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        style! {St::Display => "flex", St::FlexDirection => "column", St::MaxWidth => px(250), St::Margin => "auto", St::MarginTop => px(30)},
        textarea![
            attrs! {At::Value => model.text},
            input_ev(Ev::Input, Msg::TextChanged),
        ],
        div![
            style! {St::Display => "flex", St::JustifyContent => "space-between", St::Padding => px(10)},
            span![IF!(calculate_hash(&model.text) != model.saved_text_hash => "Unsaved changes")],
            button![ev(Ev::Click, |_| Msg::Save), "Save"],
        ],
        div![
            style! {St::Display => "flex", St::JustifyContent => "space-between"},
            a![
                attrs! {At::Href => Urls::new(&model.base_url).home()},
                "Home"
            ],
            a![
                attrs! {At::Href => Urls::new(&model.base_url).no_home()},
                "/no-home"
            ],
            a![attrs! {At::Href => "https://example.com"}, "example.com"],
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
