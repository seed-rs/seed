use itertools::Itertools;
use js_sys;
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;

mod counter;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlRequested)
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url))
        .stream(streams::window_event(Ev::Resize, |_| Msg::OnResize));

    Model {
        sub_handles: Vec::new(),
        timer_handle: None,
        timeout_handle: None,
        seconds: 0,
        counter: counter::init(&mut orders.proxy(Msg::Counter)),
        window_size: window_size(),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    sub_handles: Vec<SubHandle>,
    timer_handle: Option<StreamHandle>,
    timeout_handle: Option<CmdHandle>,
    seconds: u32,
    counter: counter::Model,
    window_size: (f64, f64),
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

    StringReceived(String),
    UrlRequested(subs::UrlRequested),
    UrlChanged(subs::UrlChanged),

    SetTimeout,
    CancelTimeout,

    OnResize,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Subscribe => {
            log!("--- Subscribe ---");
            model.sub_handles = vec![
                orders.subscribe_with_handle(|number: i32| log!("Number Received", number)),
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
        Msg::StringReceived(message) => {
            log!("String Received", message);
        }
        Msg::UrlRequested(subs::UrlRequested(url, _url_request)) => {
            log!("Url Requested", url);
        }
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("Url Changed", url);
        }
        Msg::SetTimeout => {
            log!("--- Set timeout ---");
            model.timeout_handle =
                Some(orders.perform_cmd_with_handle(cmds::timeout(2000, || log!("Timeout!!"))));
        }
        Msg::CancelTimeout => {
            log!("--- Cancel timeout ---");
            model.timeout_handle = None;
        }
        Msg::OnResize => {
            model.window_size = window_size();
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    let centered_column = style! {
        St::Display => "flex",
        St::FlexDirection => "column",
        St::AlignItems => "center"
    };

    div![
        centered_column.clone(),
        "Open Console log, please",
        divider(),
        // --- Subscribe | Notify | Unsubscribe ---
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
                ev(Ev::Click, |_| log!("Reset counter!")),
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
        ],
        divider(),
        // --- Timeout ---
        div![
            style! {St::Display => "flex"},
            with_spaces(vec![
                button![ev(Ev::Click, |_| Msg::SetTimeout), "Set 2s timeout"],
                button![ev(Ev::Click, |_| Msg::CancelTimeout), "Cancel"],
            ]),
        ],
        divider(),
        // --- Window size ---
        {
            let (width, height) = &model.window_size;
            format!("Window size: {} x {}", width, height)
        }
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
//    Helpers
// ------ ------

fn window_size() -> (f64, f64) {
    let window = window();
    let width = window
        .inner_width()
        .expect("window width")
        .unchecked_into::<js_sys::Number>()
        .value_of();
    let height = window
        .inner_height()
        .expect("window height")
        .unchecked_into::<js_sys::Number>()
        .value_of();
    (width, height)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
