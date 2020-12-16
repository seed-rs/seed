use crate::{
    models::{
        auth::LoginCredentials,
        user::{LoggedData, Role},
    },
    request::State,
};
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
    request_state: State<LoggedData>,
}

pub enum Msg {
    AutoLogin(Role),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AutoLogin(role) => {
            let logged_user = match role {
                Role::StandardUser => LoggedData::new(
                    "John",
                    "Doe",
                    "JohnUnknown",
                    "unknown@gmail.com",
                    Role::StandardUser,
                ),
                Role::Admin => LoggedData::new(
                    "Janne",
                    "Doe",
                    "JanneUnknown",
                    "unknown@gmail.com",
                    Role::Admin,
                ),
            };
            model.request_state = State::Success(logged_user.clone());
            orders.notify(logged_user);
        }
    }
}
pub fn view(model: &Model) -> Node<Msg> {
    match &model.request_state {
        State::Success(user) => div![p![
            C!["centred"],
            "Welcome ",
            style! {St::Color => "darkblue"},
            user.username(),
            ". :)"
        ]],
        State::IsPending(status) => form(model, status),
    }
}

fn form(model: &Model, status: &bool) -> Node<Msg> {
    form![
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
