#![allow(clippy::must_use_candidate)]

use seed::{prelude::*, *};

mod page;

const ADMIN: &str = "admin";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url.clone()));

    Model {
        ctx: Context {
            logged_user: "John Doe",
        },
        base_url: url.truncate_relative_path(),
        page_id: None,
        admin_model: None,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    base_url: Url,
    page_id: Option<PageId>,
    admin_model: Option<page::admin::Model>,
}

// ------ Context ------

pub struct Context {
    pub logged_user: &'static str,
}

// ------ PageId ------

#[derive(Copy, Clone, Eq, PartialEq)]
enum PageId {
    Home,
    Admin,
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
        page::admin::Urls::new(self.base_url().push_path_part(ADMIN))
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
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.page_id = match url.pop_relative_path_part() {
                None => Some(PageId::Home),
                Some(ADMIN) => {
                    page::admin::init(url, &mut model.admin_model).map(|_| PageId::Admin)
                }
                _ => None,
            };
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        header(&model.base_url),
        match model.page_id {
            Some(PageId::Home) => div!["Welcome home!"],
            Some(PageId::Admin) => {
                page::admin::view(model.admin_model.as_ref().expect("admin model"), &model.ctx)
            }
            None => div!["404"],
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
            attrs! { At::Href => Urls::new(base_url).admin_urls().report_urls().root() },
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
