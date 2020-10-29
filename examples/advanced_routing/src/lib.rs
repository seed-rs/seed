mod request;
use seed::{prelude::*, *};
extern crate heck;
use crate::{
    models::user::{LoggedUser, Role},
    theme::Theme,
    top_bar::TopBar,
};
#[macro_use]
extern crate seed_routing;
use crate::pages::dashboard::{tasks::TasksRoutes, DashboardRoutes};
use seed_routing::View;
use seed_routing::*;
pub mod models;
mod pages;
mod theme;
mod top_bar;
use crate::pages::admin::AdminRoutes;

use std::fmt::Debug;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .subscribe(Msg::UrlRequested)
        .subscribe(Msg::UserLogged);

    let mut router: Router<Routes> = Router::new();
    router.init_url_and_navigation(url);

    Model {
        theme: Theme::default(),
        login: Default::default(),
        dashboard: Default::default(),
        admin: Default::default(),
        router,
        logged_user: None,
    }
}
#[derive(Debug, PartialEq, Clone, RoutingModules)]
#[modules_path = "pages"]
pub enum Routes {
    Login {
        query: IndexMap<String, String>, // -> http://localhost:8000/login?name=JohnDoe
    },
    #[guard = " => guard => forbidden"]
    Dashboard(DashboardRoutes), // -> http://localhost:8000/dashboard/*
    #[guard = " => admin_guard => forbidden_user"]
    Admin {
        // -> /admin/:id/*
        id: String,
        children: AdminRoutes,
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
            "Sorry {} {} , but you are missing the Admin Role. Ask your administrator for more \
             information. ",
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
    router: Router<Routes>,
    logged_user: Option<LoggedUser>,
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
    UrlRequested(subs::UrlRequested),
    Login(pages::login::Msg),
    Admin(pages::admin::Msg),
    UserLogged(LoggedUser),
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
            model.router.confirm_navigation(url);
            if let Some(current_route) = model.router.current_route.clone() {
                current_route.init(model, orders);
            }
        }
        Msg::UrlRequested(_) => {}
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
            model
                .router
                .request_moving_back(|r| orders.notify(subs::UrlRequested::new(r)));
        }
        Msg::GoForward => {
            model
                .router
                .request_moving_forward(|r| orders.notify(subs::UrlRequested::new(r)));
        }
        Msg::Logout => model.logged_user = None,
        Msg::GoLogin => {
            model.router.current_route = Some(Routes::Login {
                query: IndexMap::new(),
            })
        }
    }
}

// ------ ------
//     View
// ------ ------
/// View function which renders stuff to html
fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        header(&model),
        if let Some(route) = &model.router.current_route {
            route.view(model)
        } else {
            home(&model.theme)
        },
    ]
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
                        At::Disabled  =>   (!model.router.can_back()).as_at_value(),
                    },
                    ev(Ev::Click, |_| Msg::GoBack)
                ],
                button![
                    "forward",
                    attrs! {
                        At::Disabled =>  (!model.router.can_forward()).as_at_value(),
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
        li![a![
            C![
                "route",
                IF!( model.router.is_current_route(&Routes::Login { query : IndexMap::new() }) => "active-route" )
            ],
            attrs! { At::Href => Routes::Login { query : IndexMap::new() }.to_url() },
            "Login",
        ]],
        li![a![
            C![
                "route",
                IF!( model.router.is_current_route(&Routes::Login { query : make_query_for_john_doe() }) => "active-route" )
            ],
            attrs! { At::Href => Routes::Login { query : make_query_for_john_doe() }.to_url()  },
            "Login for JohnDoe",
        ]],
        li![a![
            C![
                "route",
                IF!(model.router.is_current_route(&Routes::NotFound) => "active-route" )
            ],
            attrs! { At::Href => &Routes::NotFound.to_url() },
            "NotFound",
        ]],
        li![a![
            C![
                "route",
                IF!(model.router.is_current_route(&Routes::Home) => "active-route" )
            ],
            attrs! { At::Href => &Routes::Home.to_url()  },
            "Home",
        ]],
        li![a![C!["route"], "Admin",]],
        ul![
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "1".to_string() , children : AdminRoutes::Root}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => &Routes::Admin { id : "1".to_string() , children : AdminRoutes::Root}.to_url()  },
                "Admin project 1",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "2".to_string() , children : AdminRoutes::Root}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => &Routes::Admin { id : "2".to_string() , children : AdminRoutes::Root}.to_url()  },
                "Admin project 2",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "3".to_string() , children : AdminRoutes::Root}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => &Routes::Admin { id : "3".to_string() , children : AdminRoutes::Root}.to_url()  },
                "Admin project 3",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "3".to_string() , children : AdminRoutes::NotFound}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => &Routes::Admin { id : "3".to_string() , children : AdminRoutes::NotFound}.to_url()  },
                "Not found project 3",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Admin { id : "1".to_string() , children : AdminRoutes::Manager}) => "active-route" ),
                    IF!(admin_guard(model).is_none() => "locked-route"   ),
                    IF!(admin_guard(model).is_some() && !admin_guard(model).unwrap() => "locked-admin-route" )
                ],
                attrs! { At::Href => &Routes::Admin { id : "1".to_string() , children : AdminRoutes::Manager}.to_url()  },
                "Manage project 1",
            ]],
        ],
        li![a![C!["route"], "Dashboard",]],
        ul![
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Root)) => "active-route" ),
                    IF!(guard(model).is_none() => "locked-route"   )
                ],
                attrs! { At::Href => &Routes::Dashboard(DashboardRoutes::Root).to_url()  },
                "Profile",
            ]],
            li![a![
                C![
                     "route",
                     IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Message)) => "active-route" )
                IF!(guard(model).is_none() => "locked-route"   )
                 ],
                attrs! { At::Href => &Routes::Dashboard(DashboardRoutes::Message).to_url()  },
                "Messages",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Statistics)) => "active-route" )
                       IF!(guard(model).is_none() => "locked-route"   )
                ],
                attrs! { At::Href => &Routes::Dashboard(DashboardRoutes::Statistics).to_url()  },
                "Statistics",
            ]],
            li![a![
                C![
                    "route",
                    IF!(model.router.is_current_route(&Routes::Dashboard(DashboardRoutes::Tasks { query: IndexMap::new() , children :  TasksRoutes::Root  })) => "active-route" )
                    IF!(guard(model).is_none() => "locked-route"   )
                ],
                attrs! { At::Href => &Routes::Dashboard(DashboardRoutes::Tasks { query: IndexMap::new() , children :  TasksRoutes::Root  }) .to_url()  },
                "Tasks",
            ]],
        ],
    ]
}

// fn cannot_user_access_dashboard(model: &Model) -> bool {
//     Routes::Dashboard(DashboardRoutes::Root)
//         .check_before_load(model)
//         .is_none()
// }

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
