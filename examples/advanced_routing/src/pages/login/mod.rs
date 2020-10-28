use crate::models::auth::LoginCredentials;
use crate::models::user::{LoggedUser, Role};
use crate::request::RequestState;
use seed::{prelude::*, *};

/// Can trigger specific update when loading the page
pub fn init(
    _: Url,
    _: &mut Model,
    query: &IndexMap<String, String>,
    _: &mut impl Orders<Msg>,
) -> Model {
    let name = query.get("name");

    if let Some(name_from_query) = name {
        let mut model = Model {
            credentials: Default::default(),
            request_state: Default::default(),
        };

        model.credentials.set_target(name_from_query.to_string());
        model
    } else {
        Model {
            credentials: Default::default(),
            request_state: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Model {
    credentials: LoginCredentials,
    request_state: RequestState<LoggedUser>,
}

pub enum Msg {
    Login,
    LoginSucceed(LoggedUser),
    AutoLogin(Role),
    LoginFailed { message: String, code: String },
    PasswordChanged(String),
    TargetChanged(String),
    Clear,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Login => {
            model.request_state = RequestState::IsPending(true);
            let request = Request::new("/api/auth")
                .method(Method::Post)
                .json(&model.credentials)
                .expect("Serialization failed");

            model.credentials.set_password("".to_string());
            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");

                if response.status().is_ok() {
                    Msg::LoginSucceed(response.json().await.unwrap())
                } else {
                    Msg::LoginFailed {
                        message: response.text().await.unwrap(),
                        code: response.status().code.to_string(),
                    }
                }
            });
        }
        Msg::Clear => {
            model.credentials = Default::default();
            model.request_state = Default::default();
        }
        Msg::LoginSucceed(logged_user) => {
            model.request_state = RequestState::Success(logged_user.clone());
            orders.notify(logged_user);
        }
        Msg::LoginFailed { message, code } => {
            model.request_state = RequestState::Failed { message, code }
        }
        Msg::PasswordChanged(pwd) => {
            model.credentials.set_password(pwd);
        }
        Msg::TargetChanged(target) => model.credentials.set_target(target),
        Msg::AutoLogin(role) => {
            let logged_user = match role {
                Role::StandardUser => LoggedUser::new(
                    "John",
                    "Doe",
                    "JohnUnknown",
                    "unknown@gmail.com",
                    Role::StandardUser,
                ),
                Role::Admin => LoggedUser::new(
                    "Janne",
                    "Doe",
                    "JanneUnknown",
                    "unknown@gmail.com",
                    Role::Admin,
                ),
            };
            model.request_state = RequestState::Success(logged_user.clone());
            orders.notify(logged_user);
        }
    }
}
pub fn view(model: &Model) -> Node<Msg> {
    match &model.request_state {
        RequestState::Success(user) => div![p![
            C!["centred"],
            "Welcome ",
            style! {St::Color => "darkblue"},
            user.username(),
            ". :)"
        ]],
        RequestState::IsPending(status) => form(model, status),
        RequestState::Failed { message, code } => p![
            C!["centred"],
            format!("An error happened {} with the code {}", message, code)
        ],
    }
}

fn form(model: &Model, status: &bool) -> Node<Msg> {
    form![
        ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::Login
        }),
        fieldset![
            attrs! {
                        At::Disabled=> status.as_at_value(),
            },
            legend!["credentials"],
            label![attrs! { At::For => "username"}, "Username/Email"],
            input![
                id!("username"),
                attrs! {
                At::Required => true,
                At::Value=> model.credentials.target(),
                At::MinLength=> "5",
                At::Name => "username",
                At::MaxLength=> "25",
                At::Type=> "text"
                        },
                input_ev(Ev::Input, Msg::TargetChanged),
            ],
            label![attrs! { At::For => "password"}, "Password"],
            input![
                id!("password"),
                attrs! {
                    At::Required => true,
                    At::MinLength=> "8",
                    At::MaxLength=> "30"
                    At::Value => model.credentials.password(),
                    At::Name => "password",
                    At::Type=> "password"
                },
                input_ev(Ev::Input, Msg::PasswordChanged),
            ],
        ],
        button![
            "Login",
            attrs! {
            At::Type=> "submit"
                    },
        ],
        IF!(*status =>  div![C!["lds-ring"], div![], div![], div![], div![]] ),
        br![],
        button![
            "Sign in as John Doe",
            ev(Ev::Click, |_| Msg::AutoLogin(Role::StandardUser)),
        ],
        br![],
        button![
            "Sign in as Janne Doe",
            ev(Ev::Click, |_| Msg::AutoLogin(Role::Admin)),
        ],
    ]
}
