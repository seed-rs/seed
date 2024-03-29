//! The simplest fetch example.

use gloo_net::http::Request;
use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    user: Option<User>,
}

#[derive(serde::Deserialize)]
pub struct User {
    name: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    Fetch,
    Received(User),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetch => {
            orders.skip(); // No need to rerender
            orders.perform_cmd(async {
                let response = Request::get("user.json")
                    .send()
                    .await
                    .expect("HTTP request failed");

                if !response.ok() {
                    // TODO: handle error
                    None
                } else {
                    let user = response
                        .json::<User>()
                        .await
                        .expect("deserialization failed");
                    Some(Msg::Received(user))
                }
            });
        }
        Msg::Received(user) => {
            model.user = Some(user);
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::Fetch), "Fetch user"],
        model
            .user
            .as_ref()
            .map(|user| div![format!("User: {}", user.name)])
    ]
}
