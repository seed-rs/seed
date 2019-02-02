use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use serde::{Deserialize, Serialize};

use crate::{util, App};

/// Contains all information used in pushing and handling routes.
/// Based on React-Reason's router:
/// https://github.com/reasonml/reason-react/blob/master/docs/router.md
#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub path: Vec<String>,
    pub hash: Option<String>,
    pub search: Option<String>,
    pub title: Option<String>,
}

impl Url {

    /// Helper that ignores hash, search and title, and converts path to Strings.
    pub fn new(path: Vec<&str>) -> Self {
        Self {
            path: path.into_iter().map(|p| p.to_string()).collect(),
            hash: None,
            search: None,
            title: None,
        }
    }

    pub fn hash(mut self, hash: &str) -> Self {
        self.hash = Some(hash.into());
        self
    }

    pub fn search(mut self, search: &str) -> Self {
        self.search = Some(search.into());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
        self
    }
}

///// A convenience function to prevent repetitions
//fn get_path() -> String {
//    let path = util::window()
//        .location()
//        .pathname()
//        .expect("Can't find pathname");
//    path[1..path.len()].to_string()
//}

/// For setting up landing page routing.
pub fn initial<Ms, Mdl>(app: App<Ms, Mdl>, routes: fn(Url) -> Ms) -> App<Ms, Mdl>
where
    Ms: Clone + 'static,
    Mdl: Clone + 'static,
{
//    let path = get_path();
    // todo this is where you start tmw
    crate::log(path);

    let url = Url {
        path: vec![],
        hash: None,
        search: None,
        title: None,
    };
//    app.update(routes(url));  // todo errors here yo

    app
}

pub fn setup_popstate_listener<Ms, Mdl>(app: &App<Ms, Mdl>, routes: fn(Url) -> Ms)
    where
        Ms: Clone,
        Mdl: Clone,
{
    // We can't reuse the app later to store the popstate once moved into the closure.
    let app_for_closure = app.clone();
    let closure = Closure::wrap(Box::new(move |ev: web_sys::Event| {

        let ev = ev.dyn_ref::<web_sys::PopStateEvent>()
            .expect("Problem casting as Popstate event");

        let state_str = ev.state().as_string()
            .expect("Problem casting state as string");

        let url: Url = serde_json::from_str(&state_str)
            .expect("Problem deserialzing popstate state");
//        crate::log(format!("{:?}", url));

        app_for_closure.update(routes(url));

    }) as Box<FnMut(web_sys::Event) + 'static>);

    (util::window().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .expect("Problem adding popstate listener");
    app.data.popstate_closure.replace(Some(closure));
}

/// Add a new route using history's push_state method.
///https://developer.mozilla.org/en-US/docs/Web/API/History_API
pub fn push_route(url: Url) {
    // We use data to evaluate the path instead of the path displayed in the url.
    let data = JsValue::from_serde(
        &serde_json::to_string(&url)
            .expect("Problem serializing route data")
    )
        .expect("Problem converting route data to JsValue");

    // title is currently unused by Firefox.
    let title = match url.title {
        Some(t) => t,
        None => "".into()
    };

    // Prepending / means replace
    // the existing path. Not doing so will add the path to the existing one.
    let mut path = String::from("/") + &url.path.join("/");

    if let Some(search) = url.search {
        path += "?";
        path += &search;
    }
    if let Some(hash) = url.hash {
        path += "#";
        path += &hash;
    }

    util::window().history()
        .expect("Can't find history")
        .push_state_with_url(&data, &title, Some(&path))
        .expect("Problem pushing state");
}

