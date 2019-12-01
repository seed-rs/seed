use seed::{*, prelude::*};
use web_sys;

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    clicks: u32,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
enum Msg {
    Clicked,
    UrlChanged(Url),
    KeyPressed(web_sys::KeyboardEvent),
    SayHello,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::Clicked => model.clicks += 1,
        Msg::UrlChanged(url) => {
            log!(url);
            orders.skip();
        },
        Msg::KeyPressed(event) => {
            log!(event.key());
            orders.skip();
        },
        Msg::SayHello => {
            orders.send_g_msg(GMsg::SayHello);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    vec![
        button![
            format!("Clicked: {}", model.clicks),
            simple_ev(Ev::Click, Msg::Clicked),
        ],
        button![
            "Say hello",
            simple_ev(Ev::Click, Msg::SayHello),
        ]
    ]
}

// ------ ------
//    Routes
// ------ ------

fn routes(url: Url) -> Option<Msg> {
    Some(Msg::UrlChanged(url))
}

// ------ ------
// Window Events
// ------ ------

fn window_events(_model: &Model) -> Vec<Listener<Msg>> {
    vec![
        keyboard_ev(Ev::KeyDown, Msg::KeyPressed)
    ]
}

// ------ ------
//     Sink
// ------ ------

enum GMsg {
    SayHello,
}

fn sink(g_msg: GMsg, _model: &mut Model, _orders: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SayHello => log!("Hello!"),
    }
}

// ------ ------
// Before Mount
// ------ ------

fn before_mount(_url: Url) -> BeforeMount {
    BeforeMount::default()
        .mount_point("main")
        .mount_type(MountType::Takeover)
}

// ------ ------
//  After Mount
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .routes(routes)
        .window_events(window_events)
        .sink(sink)
        .before_mount(before_mount)
//        .after_mount(after_mount)
        .build_and_start();
}
