#![allow(clippy::must_use_candidate)]

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

const LOGIN: &str = "login";
const API_URL: &str = "https://martinkavik-seed-auth-example.builtwithdark.com/api";
const STORAGE_KEY: &str = "seed_auth_example";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    let user = LocalStorage::get(STORAGE_KEY).ok();
    Model {
        email: "john@example.com".to_owned(),
        password: "1234".to_owned(),
        base_url: url.clone().truncate_relative_path(),
        page: Page::init(url, user.as_ref(), orders),
        secret_message: None,
        user,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    email: String,
    password: String,
    base_url: Url,
    page: Page,
    secret_message: Option<String>,
    user: Option<LoggedUser>,
}

// ------ LoggedUser ------

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LoggedUser {
    id: usize,
    email: String,
    username: String,
    token: String,
}

// ------ Page ------

enum Page {
    Home,
    Login,
    NotFound,
}

impl Page {
    fn init(mut url: Url, user: Option<&LoggedUser>, orders: &mut impl Orders<Msg>) -> Self {
        match url.pop_relative_path_part() {
            None => {
                if let Some(user) = user {
                    send_request_to_top_secret(user.token.clone(), orders)
                };
                Self::Home
            }
            Some(LOGIN) => Self::Login,
            _ => Self::NotFound,
        }
    }
}

fn send_request_to_top_secret(token: String, orders: &mut impl Orders<Msg>) {
    orders.perform_cmd(async {
        Msg::TopSecretFetched(
            async {
                Request::new(format!("{}/top_secret", API_URL))
                    .header(Header::bearer(token))
                    .fetch()
                    .await?
                    .check_status()?
                    .text()
                    .await
            }
            .await,
        )
    });
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn login(self) -> Url {
        self.base_url().push_path_part(LOGIN)
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
    EmailChanged(String),
    PasswordChanged(String),
    LoginClicked,
    LoginFetched(fetch::Result<LoggedUser>),
    TopSecretFetched(fetch::Result<String>),
    LogoutClicked,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url, model.user.as_ref(), orders);
        }
        Msg::EmailChanged(email) => model.email = email,
        Msg::PasswordChanged(password) => model.password = password,
        Msg::LoginClicked => {
            let request = Request::new(format!("{}/users/login", API_URL))
                .method(Method::Post)
                .json(&LoginRequestBody {
                    email: &model.email,
                    password: &model.password,
                });
            orders.perform_cmd(async {
                Msg::LoginFetched(
                    async { request?.fetch().await?.check_status()?.json().await }.await,
                )
            });
        }
        Msg::LoginFetched(Ok(logged_user)) => {
            LocalStorage::insert(STORAGE_KEY, &logged_user).expect("save user");
            model.user = Some(logged_user);
            orders.notify(subs::UrlRequested::new(Urls::new(&model.base_url).home()));
        }
        Msg::TopSecretFetched(Ok(secret_message)) => {
            model.secret_message = Some(secret_message);
        }
        Msg::LoginFetched(Err(error)) | Msg::TopSecretFetched(Err(error)) => log!(error),
        Msg::LogoutClicked => {
            LocalStorage::remove(STORAGE_KEY).expect("remove saved user");
            model.user = None;
            model.secret_message = None;
        }
    }
}

#[derive(Serialize)]
struct LoginRequestBody<'a> {
    email: &'a str,
    password: &'a str,
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        header(&model.base_url, model.user.as_ref()),
        match &model.page {
            Page::Home => div![
                format!(
                    "Welcome home {}!",
                    model
                        .user
                        .as_ref()
                        .map(|user| user.username.to_owned())
                        .unwrap_or_default()
                ),
                div![&model.secret_message],
            ],
            Page::Login => form![
                style! {
                    St::Display => "flex",
                    St::FlexDirection => "column",
                },
                label!["Email"],
                input![
                    attrs! {
                        At::Value => model.email
                    },
                    input_ev(Ev::Input, Msg::EmailChanged)
                ],
                label!["Password"],
                input![
                    attrs! {
                        At::Value => model.password,
                        At::Type => "password",
                    },
                    input_ev(Ev::Input, Msg::PasswordChanged)
                ],
                button![
                    "Login",
                    ev(Ev::Click, |event| {
                        event.prevent_default();
                        Msg::LoginClicked
                    })
                ],
                "Note: Errors are logged into the console log.",
            ],
            Page::NotFound => div!["404"],
        },
    ]
}

fn header(base_url: &Url, user: Option<&LoggedUser>) -> Node<Msg> {
    ul![
        li![a![
            attrs! { At::Href => Urls::new(base_url).home() },
            "Home",
        ]],
        if user.is_none() {
            li![a![
                attrs! { At::Href => Urls::new(base_url).login() },
                "Login",
            ]]
        } else {
            li![a![
                attrs! { At::Href => "" },
                "Logout",
                ev(Ev::Click, |_| Msg::LogoutClicked),
            ]]
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
