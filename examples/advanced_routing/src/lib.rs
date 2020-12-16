mod request;
use seed::{prelude::*, *};
extern crate heck;
use crate::{
    models::user::{LoggedData, Role},
    theme::Theme,
    top_bar::TopBar,
};
#[macro_use]
extern crate seed_routing;
use seed_routing::{View, *};
pub mod models;
mod pages;
mod theme;
mod top_bar;
use std::fmt::Debug;

add_router!();
// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged).subscribe(Msg::UserLogged);

    router()
        .init(url)
        .set_handler(orders, move |subs::UrlRequested(requested_url, _)| {
            router().confirm_navigation(requested_url)
        });

    Model {
        theme: Theme::default(),
        login: Default::default(),
        dashboard: Default::default(),
        admin: Default::default(),
        logged_user: None,
    }
}
#[derive(Debug, PartialEq, Clone, RoutingModules)]
#[modules_path = "pages"]
pub enum Route {
    Login {
        query: IndexMap<String, String>, // -> http://localhost:8000/login?name=JohnDoe
    },
    #[guard = " => guard => forbidden"]
    Dashboard(pages::dashboard::Routes), // -> http://localhost:8000/dashboard/*
    #[guard = " => admin_guard => forbidden_user"]
    Admin {
        // -> /admin/:id/*
        id: String,
        children: pages::admin::Routes,
    },
    #[default_route]
    #[view = " => not_found"] // -> http://localhost:8000/not_found*
    NotFound,
    #[view = " => forbidden"] // -> http://localhost:8000/forbidden*
    Forbidden,
    #[as_path = ""]
    #[view = "theme => home"] // -> http://localhost:8000/
    Home,
}

fn guard(model: &Model) -> Option<bool> {
    // could check local storage, cookie or what ever you want
    if model.logged_user.is_some() {
        Some(true)
    } else {
        None
    }
}

fn admin_guard(model: &Model) -> Option<bool> {
    // could check local storage, cookie or what ever you want
    if let Some(user) = &model.logged_user {
        match user.role {
            Role::StandardUser => Some(false),
            Role::Admin => Some(true),
        }
    } else {
        None
    }
}

fn not_found(_: &Model) -> Node<Msg> {
    div!["404 page not found"]
}

fn forbidden(_: &Model) -> Node<Msg> {
    div!["401 access denied"]
}

fn forbidden_user(model: &Model) -> Node<Msg> {
    if let Some(user) = &model.logged_user {
        p![format!(
            "Sorry {} {} , but you are missing the Admin Role. Ask your administrator for more information. ",
            user.first_name, user.last_name
        )]
    } else {
        div!["401"]
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    pub login: pages::login::Model,
    pub dashboard: pages::dashboard::Model,
    pub admin: pages::admin::Model,
    logged_user: Option<LoggedData>,
    theme: Theme,
}

// ------ ------
//    Update
// ------ ------
/// Root actions for your app.
/// Each component will have single action/message mapped to its message later
/// in update

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    Login(pages::login::Msg),
    Admin(pages::admin::Msg),
    UserLogged(LoggedData),
    Dashboard(pages::dashboard::Msg),
    GoBack,
    GoForward,
    Logout,
    GoLogin,
    SwitchToTheme(Theme),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            router().current_route().init(model, orders);
        }
        Msg::Login(login_message) => pages::login::update(
            login_message,
            &mut model.login,
            &mut orders.proxy(Msg::Login),
        ),
        Msg::Dashboard(dashboard_message) => pages::dashboard::update(
            dashboard_message,
            &mut model.dashboard,
            &mut orders.proxy(Msg::Dashboard),
        ),

        Msg::Admin(admin_msg) => {
            pages::admin::update(admin_msg, &mut model.admin, &mut orders.proxy(Msg::Admin))
        }
        Msg::UserLogged(user) => {
            model.logged_user = Some(user);
        }

        Msg::SwitchToTheme(theme) => model.theme = theme,

        Msg::GoBack => {
            router().request_moving_back(|r| orders.notify(subs::UrlRequested::new(r)));
        }
        Msg::GoForward => {
            router().request_moving_forward(|r| orders.notify(subs::UrlRequested::new(r)));
        }
        Msg::Logout => model.logged_user = None,
        Msg::GoLogin => {
            // model.router.current_route = Some(Routes::Login {
            //     query: IndexMap::new(),
            // },)
        }
    }
}

// ------ ------
//     View
// ------ ------
/// View function which renders stuff to html
fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![header(&model), router().current_route().view(model)]
}

fn header(model: &Model) -> Node<Msg> {
    div![
        TopBar::new(who_is_connected(model))
            .style(model.theme.clone())
            .set_user_login_state(model.logged_user.is_some())
            .content(div![
                style! {St::Display => "flex" },
                button![
                    "back",
                    attrs! {
                        At::Disabled  =>   (!  router().can_back()).as_at_value(),
                    },
                    ev(Ev::Click, |_| Msg::GoBack)
                ],
                button![
                    "forward",
                    attrs! {
                        At::Disabled =>  (!  router().can_forward()).as_at_value(),
                    },
                    ev(Ev::Click, |_| Msg::GoForward)
                ],
                span![style! {St::Flex => "5" },],
                build_account_button(model.logged_user.is_some())
            ]),
        render_route(model)
    ]
}

fn who_is_connected(model: &Model) -> String {
    if let Some(user) = &model.logged_user {
        let full_welcome = format!("Welcome {} {}", user.first_name, user.last_name);
        full_welcome
    } else {
        "Welcome Guest".to_string()
    }
}

fn build_account_button(user_logged_in: bool) -> Node<Msg> {
    if user_logged_in {
        span![button![
            "logout ",
            ev(Ev::Click, |_| Msg::Logout),
            C!["user_button"],
            i![C!["far fa-user-circle"]]
        ]]
    } else {
        span![button![
            "sign in ",
            ev(Ev::Click, |_| Msg::GoLogin),
            C!["user_button"],
            i![C!["fas fa-user-circle"]]
        ]]
    }
}

fn make_query_for_john_doe() -> IndexMap<String, String> {
    let mut query: IndexMap<String, String> = IndexMap::new();
    query.insert("name".to_string(), "JohnDoe".to_string());
    query
}

fn render_route(model: &Model) -> Node<Msg> {
    ul![
        generate_root_nodes(),
        li![a![C!["route"], "Admin",]],
        ul![generate_admin_nodes(&model)],
        li![a![C!["route"], "Dashboard",]],
        ul![generate_dashboard_nodes(&model)],
    ]
}

fn generate_root_routes() -> Vec<(Route, &'static str)> {
    let mut vec: Vec<(Route, &'static str)> = vec![];
    vec.push((
        Route::Login {
            query: IndexMap::new(),
        },
        "Login",
    ));
    vec.push((
        Route::Login {
            query: make_query_for_john_doe(),
        },
        "Login for JohnDoe",
    ));
    vec.push((Route::NotFound, "NotFound"));
    vec.push((Route::Home, "Home"));
    vec
}

fn generate_root_nodes() -> Vec<Node<Msg>> {
    let mut list: Vec<Node<Msg>> = vec![];
    for route in generate_root_routes().iter() {
        list.push(li![a![
            C![
                "route",
                IF!(    router().is_current_route(&route.0 ) => "active-route" )
            ],
            attrs! { At::Href => &route.0.to_url() },
            route.1,
        ]])
    }
    list
}

fn generate_admin_routes() -> Vec<(Route, &'static str)> {
    let mut vec: Vec<(Route, &'static str)> = vec![];
    vec.push((
        Route::Admin {
            id: "1".to_string(),
            children: pages::admin::Routes::Root,
        },
        "Admin Project 1",
    ));
    vec.push((
        Route::Admin {
            id: "2".to_string(),
            children: pages::admin::Routes::Root,
        },
        "Admin Project 2",
    ));
    vec.push((
        Route::Admin {
            id: "3".to_string(),
            children: pages::admin::Routes::Root,
        },
        "Admin Project 3",
    ));
    vec.push((
        Route::Admin {
            id: "3".to_string(),
            children: pages::admin::Routes::NotFound,
        },
        "Not found project 3",
    ));
    vec.push((
        Route::Admin {
            id: "1".to_string(),
            children: pages::admin::Routes::Manager,
        },
        "Manage project 1",
    ));
    vec
}

fn generate_admin_nodes(model: &Model) -> Vec<Node<Msg>> {
    let mut list: Vec<Node<Msg>> = vec![];
    for route in generate_admin_routes().iter() {
        list.push(li![a![
            C![
                "route",
                IF!(    router().is_current_route(&route.0 ) => "active-route")
                           IF!(admin_guard(model).is_none() => "locked-route"),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap()
                    => "locked-admin-route" )
            ],
            attrs! { At::Href => &route.0.to_url() },
            route.1,
        ]])
    }
    list
}

fn generate_dashboard_routes() -> Vec<(Route, &'static str)> {
    let mut vec: Vec<(Route, &'static str)> = vec![];
    vec.push((Route::Dashboard(pages::dashboard::Routes::Root), "Profile"));
    vec.push((
        Route::Dashboard(pages::dashboard::Routes::Message),
        "Message",
    ));
    vec.push((
        Route::Dashboard(pages::dashboard::Routes::Statistics),
        "Statistics",
    ));
    vec.push((
        Route::Dashboard(pages::dashboard::Routes::Tasks {
            query: IndexMap::new(),
            children: pages::dashboard::tasks::Routes::Root,
        }),
        "Tasks",
    ));
    vec.push((
        Route::Dashboard(pages::dashboard::Routes::Tasks {
            query: make_query(),
            children: pages::dashboard::tasks::Routes::Root,
        }),
        "Tasks with url query",
    ));
    vec
}

fn generate_dashboard_nodes(model: &Model) -> Vec<Node<Msg>> {
    let mut list: Vec<Node<Msg>> = vec![];
    for route in generate_dashboard_routes().iter() {
        list.push(li![a![
            C![
                "route",
                IF!(   router().is_current_route(&route.0 ) => "active-route" )
                           IF!(guard(model).is_none() => "locked-route"   ),
            ],
            attrs! { At::Href => &route.0.to_url() },
            route.1,
        ]])
    }
    list
}

fn make_query() -> IndexMap<String, String> {
    let mut index_map: IndexMap<String, String> = IndexMap::new();
    index_map.insert("select1".to_string(), "1".to_string());
    index_map
}

fn home(theme: &Theme) -> Node<Msg> {
    div![
        div!["Welcome home!"],
        match theme {
            Theme::Dark => {
                button![
                    "Switch to Light",
                    ev(Ev::Click, |_| Msg::SwitchToTheme(Theme::Light))
                ]
            }
            Theme::Light => {
                button![
                    "Switch to Dark",
                    ev(Ev::Click, |_| Msg::SwitchToTheme(Theme::Dark))
                ]
            }
        }
    ]
}
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
