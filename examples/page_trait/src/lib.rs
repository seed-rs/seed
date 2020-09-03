use seed::{prelude::*, *};
use std::any::Any;

mod page;
use page::*;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        page: Page::from(MyPage::new(&mut orders.proxy(Msg::Page))),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    page: Page,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Page(Box<dyn Any>),
    SendText,
    ShowMyPage,
    ShowMyPage2,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Page(msg) => model.page.update(msg, &mut orders.proxy(Msg::Page)),
        Msg::SendText => {
            orders.notify("I'm coming from the root.");
        }
        Msg::ShowMyPage => model.page = Page::from(MyPage::new(&mut orders.proxy(Msg::Page))),
        Msg::ShowMyPage2 => model.page = Page::from(MyPage2::new(&mut orders.proxy(Msg::Page))),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        div![
            button!["Show MyPage", ev(Ev::Click, |_| Msg::ShowMyPage)],
            button!["Show MyPage2", ev(Ev::Click, |_| Msg::ShowMyPage2)],
            " ",
            button!["Send text", ev(Ev::Click, |_| Msg::SendText)],
        ],
        hr![],
        model.page.view().map_msg(Msg::Page)
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
