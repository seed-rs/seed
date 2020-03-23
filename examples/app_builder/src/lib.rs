use seed::{prelude::*, *};
use web_sys;

// ------ ------
// Before Mount
// ------ ------

fn before_mount(_: Url) -> BeforeMount {
    BeforeMount::new()
        .mount_point("main")
        .mount_type(MountType::Takeover)
}

// ------ ------
//     Model
// ------ ------

struct Model {
    clicks: u32,
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(_: Url, _: &mut impl Orders<Msg, GMsg>) -> AfterMount<Model> {
    let model = Model { clicks: 0 };
    AfterMount::new(model).url_handling(UrlHandling::None)
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

fn window_events(_: &Model) -> Vec<EventHandler<Msg>> {
    vec![keyboard_ev(Ev::KeyDown, Msg::KeyPressed)]
}

// ------ ------
//     Sink
// ------ ------

#[derive(Clone, Copy)]
enum GMsg {
    SayHello,
}

fn sink(g_msg: GMsg, _: &mut Model, _: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SayHello => log!("Hello!"),
    }
}

// ------ ------
//    Update
// ------ ------

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
        }
        Msg::KeyPressed(event) => {
            log!(event.key());
            orders.skip();
        }
        Msg::SayHello => {
            orders.send_g_msg(GMsg::SayHello);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        button![
            format!("Clicked: {}", model.clicks),
            ev(Ev::Click, |_| Msg::Clicked),
        ],
        button!["Say hello", ev(Ev::Click, |_| Msg::SayHello),],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(routes)
        .window_events(window_events)
        .sink(sink)
        .build_and_start();
}
