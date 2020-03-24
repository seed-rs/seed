//! The simplest fetch example.

use seed::{prelude::*, *};

#[derive(serde::Deserialize)]
pub struct User {
    name: String,
}

#[derive(Default)]
pub struct Model {
    user: Option<User>,
}

pub enum Msg {
    Fetch,
    Received(User),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetch => {
            orders.skip(); // No need to rerender
            orders.perform_cmd(async {
                let response = fetch("user.json").await.expect("HTTP request failed");

                let user = response
                    .check_status() // ensure we've got 2xx status
                    .expect("status check failed")
                    .json::<User>()
                    .await
                    .expect("deserialization failed");

                Msg::Received(user)
            });
        }
        Msg::Received(user) => {
            model.user = Some(user);
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::Fetch), "Fetch user"],
        model
            .user
            .as_ref()
            .map(|user| div![format!("User: {}", user.name)])
    ]
}
