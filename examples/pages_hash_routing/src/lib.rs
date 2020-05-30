#![allow(clippy::must_use_candidate)]

use seed::{prelude::*, *};

mod page;

const ADMIN: &str = "admin";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        ctx: Context {
            logged_user: "John Doe",
        },
        base_url: url.clone().truncate_relative_hash_path(),
        page: Page::init(url),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    base_url: Url,
    page: Page,
}

// ------ Context ------

pub struct Context {
    pub logged_user: &'static str,
}

// ------ Page ------

enum Page {
    Home,
    Admin(page::admin::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.pop_relative_hash_path_part() {
            None => Self::Home,
            Some(ADMIN) => page::admin::init(url).map_or(Self::NotFound, Self::Admin),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn admin_urls(self) -> page::admin::Urls<'a> {
        page::admin::Urls::new(self.base_url().push_hash_path_part(ADMIN))
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        header(&model.base_url),
        match &model.page {
            Page::Home => div!["Welcome home!"],
            Page::Admin(admin_model) => page::admin::view(admin_model, &model.ctx),
            Page::NotFound => div!["404"],
        },
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    ul![
        li![a![
            attrs! { At::Href => Urls::new(base_url).home() },
            "Home",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).admin_urls().report_urls().default() },
            "Report",
        ]],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
