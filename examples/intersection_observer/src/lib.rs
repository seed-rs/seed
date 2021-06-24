use seed::{prelude::*, *};
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::SetupObserver);
    Model {
        box_container: ElRef::new(),
        red_box: ElRef::new(),
        observer: None,
        observer_callback: None,
        observer_entries: None,
    }
}

struct Model {
    box_container: ElRef<web_sys::Element>,
    red_box: ElRef<web_sys::Element>,
    observer: Option<IntersectionObserver>,
    observer_callback: Option<Closure<dyn Fn(Vec<JsValue>)>>,
    observer_entries: Option<Vec<IntersectionObserverEntry>>,
}

enum Msg {
    SetupObserver,
    Observed(Vec<IntersectionObserverEntry>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Observed(entries) => {
            model.observer_entries = Some(entries);
        }
        Msg::SetupObserver => {
            orders.skip();

            // ---- observer callback ----
            let sender = orders.msg_sender();
            let callback = move |entries: Vec<JsValue>| {
                let entries = entries
                    .into_iter()
                    .map(IntersectionObserverEntry::from)
                    .collect();
                sender(Some(Msg::Observed(entries)));
            };
            let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn(Vec<JsValue>)>);

            // ---- observer options ----
            let mut options = IntersectionObserverInit::new();
            options.threshold(&JsValue::from(1));
            // ---- observer ----
            let observer =
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                    .unwrap();

            observer.observe(&model.red_box.get().unwrap());

            // Note: Drop `observer` is not enough. We have to call `observer.disconnect()`.
            model.observer = Some(observer);
            model.observer_callback = Some(callback);
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        view_info(model.observer_entries.as_ref()),
        view_box_container(model),
    ]
}

fn view_info(observer_entries: Option<&Vec<IntersectionObserverEntry>>) -> Option<Node<Msg>> {
    let entry = observer_entries?.first()?;
    Some(div![
        style! {
            St::Position => "fixed",
            St::Background => "white",
        },
        div![format!("Target: {}", entry.target().id()),],
        div![format!(
            "Is completely visible: {}",
            entry.is_intersecting()
        )]
    ])
}

fn view_box_container(model: &Model) -> Node<Msg> {
    div![
        // don't allow VDOM to reuse the `div` for other elements (it would break observing)
        el_key(&"box-container"),
        el_ref(&model.box_container),
        style! {
            St::Height => vh(100),
        },
        view_blue_box(),
        view_red_box(&model.red_box),
        view_blue_box(),
    ]
}

fn view_red_box(red_box: &ElRef<web_sys::Element>) -> Node<Msg> {
    div![
        el_ref(red_box),
        id!("red-box"),
        style! {
            St::Height => vh(70),
            St::Background => "red",
        }
    ]
}

fn view_blue_box() -> Node<Msg> {
    div![
        C!["blue-box"],
        style! {
            St::Height => vh(100),
            St::Background => "blue",
        }
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
