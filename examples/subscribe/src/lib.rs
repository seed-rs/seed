use itertools::Itertools;
use seed::{prelude::*, *};

mod counter;

// ------ ------
//     Model
// ------ ------

struct Model {
    sub_handles: Vec<SubHandle>,
    counter: counter::Model,
}

// ------ ------
//  AfterMount
// ------ ------

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    AfterMount::new(Model {
        sub_handles: Vec::new(),
        counter: counter::init(&mut orders.proxy(Msg::Counter)),
    })
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Subscribe,
    Notify,
    Unsubscribe,

    Counter(counter::Msg),
    ResetCounter,

    NumberReceived(i32),
    StringReceived(String),
    UrlRequested(subs::UrlRequested),
    UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Subscribe => {
            log!("--- Subscribe ---");
            model.sub_handles = vec![
                orders.subscribe(Msg::NumberReceived),
                orders.subscribe(Msg::StringReceived),
                orders.subscribe(Msg::StringReceived),
                orders.subscribe(Msg::UrlRequested),
                orders.subscribe(Msg::UrlChanged),
            ];
        }
        Msg::Notify => {
            log!("--- Notify ---");
            orders.notify(15).notify(21).notify("Hello!".to_owned());
        }
        Msg::Unsubscribe => {
            log!("--- Unsubscribe ---");
            model.sub_handles.clear();
        }
        Msg::Counter(msg) => counter::update(msg, &mut model.counter),
        Msg::ResetCounter => {
            orders.notify(counter::DoReset);
        }
        Msg::NumberReceived(message) => {
            log!("Number Received", message);
        }
        Msg::StringReceived(message) => {
            log!("String Received", message);
        }
        Msg::UrlRequested(subs::UrlRequested(url, _url_request)) => {
            log!("Url Requested", url);
        }
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("Url Changed", url);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    let centered_column = style! {
        St::Display => "flex",
        St::FlexDirection => "column",
        St::AlignItems => "center"
    };

    div![
        centered_column.clone(),
        "Open Console log, please",
        div![
            style! {St::Margin => rem(2)},
            vec![
                button![ev(Ev::Click, |_| Msg::Subscribe), "1. Subscribe"],
                button![ev(Ev::Click, |_| Msg::Notify), "2. Notify"],
                a![attrs! {At::Href => "/requested_url"}, "3. Request new URL"],
                button![ev(Ev::Click, |_| Msg::Unsubscribe), "4. Unsubscribe"],
            ]
            .into_iter()
            .intersperse(span![
                style! {St::Width => rem(1), St::Display => "inline-block"}
            ])
        ],
        div![
            centered_column,
            counter::view(&model.counter).map_msg(Msg::Counter),
            button![
                style! {St::MarginTop => rem(0.5)},
                ev(Ev::Click, |_| Msg::ResetCounter),
                "Reset counter"
            ],
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
