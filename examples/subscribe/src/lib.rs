use itertools::Itertools;
use seed::{prelude::*, *};

mod counter;

// ------ ------
//     Model
// ------ ------

struct Model {
    sub_handles: Vec<SubHandle>,
    timer_handle: Option<StreamHandle>,
    seconds: u32,
    counter: counter::Model,
}

// ------ ------
//  AfterMount
// ------ ------

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders
        .subscribe(Msg::UrlRequested)
        .subscribe(Msg::UrlChanged);

    AfterMount::new(Model {
        sub_handles: Vec::new(),
        seconds: 0,
        timer_handle: None,
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

    StartTimer,
    StopTimer,
    OnTick,

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
                orders.subscribe_with_handle(Msg::NumberReceived),
                orders.subscribe_with_handle(Msg::StringReceived),
                orders.subscribe_with_handle(Msg::StringReceived),
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
        Msg::StartTimer => {
            model.timer_handle =
                Some(orders.stream_with_handle(streams::interval(1000, || Msg::OnTick)))
        }
        Msg::StopTimer => {
            model.timer_handle = None;
        }
        Msg::OnTick => {
            model.seconds += 1;
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
        divider(),
        // --- Subscribe | Notify | Unsubscribe
        div![with_spaces(vec![
            button![ev(Ev::Click, |_| Msg::Subscribe), "1. Subscribe"],
            button![ev(Ev::Click, |_| Msg::Notify), "2. Notify"],
            button![ev(Ev::Click, |_| Msg::Unsubscribe), "3. Unsubscribe"],
        ]),],
        divider(),
        // --- Request new URL ---
        a![attrs! {At::Href => "/requested_url"}, "Request new URL"],
        divider(),
        // --- Counter ---
        div![
            centered_column,
            counter::view(&model.counter).map_msg(Msg::Counter),
            button![
                style! {St::MarginTop => rem(0.5)},
                ev(Ev::Click, |_| Msg::ResetCounter),
                "Reset counter"
            ],
        ],
        divider(),
        // --- Seconds ---
        div![
            style! {St::Display => "flex"},
            with_spaces(vec![
                div!["Seconds: ", model.seconds,],
                button![ev(Ev::Click, |_| Msg::StartTimer), "Start"],
                button![ev(Ev::Click, |_| Msg::StopTimer), "Stop"],
            ]),
        ]
    ]
}

fn divider() -> Node<Msg> {
    div![style! {St::Margin => rem(2)}]
}

fn with_spaces(nodes: Vec<Node<Msg>>) -> impl Iterator<Item = Node<Msg>> {
    nodes.into_iter().intersperse(span![
        style! {St::Width => rem(1), St::Display => "inline-block"}
    ])
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
