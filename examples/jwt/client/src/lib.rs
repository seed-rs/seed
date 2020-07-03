use seed::{prelude::*, *};

const AUTH_SERVER: &str = "http://localhost:8081";

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::LoginStatusFetched(fetch_login_status().await) });

    Model {
        login_status: LoginStatus::Fetching,
    }
}

async fn fetch_login_status() -> LoginStatus {
    let fetch_login_status = async {
        Request::new(&format!("{}/signed-in", AUTH_SERVER))
            // We have to allow cookies to be sent.
            .credentials(web_sys::RequestCredentials::Include)
            .fetch()
            .await?
            .json::<bool>()
            .await
    };

    match fetch_login_status.await {
        Ok(true) => LoginStatus::LoggedIn,
        Ok(false) => LoginStatus::NotLoggedIn,
        Err(fetch_error) => {
            log!(fetch_error);
            LoginStatus::FetchFailed(fetch_error)
        }
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    login_status: LoginStatus,
}

enum LoginStatus {
    LoggedIn,
    NotLoggedIn,
    Fetching,
    FetchFailed(FetchError),
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    LoginStatusFetched(LoginStatus),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoginStatusFetched(login_status) => model.login_status = login_status,
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        p![
            "Use this link to toggle your login status. ",
            "Close the tab and come back. Note the status should be saved."
        ],
        match model.login_status {
            LoginStatus::LoggedIn => {
                a![
                    "Sign Out",
                    attrs! {At::Href => format!("{}/sign-out", AUTH_SERVER)}
                ]
            }
            LoginStatus::NotLoggedIn => {
                a![
                    "Sign In",
                    attrs! {At::Href => format!("{}/sign-in", AUTH_SERVER)}
                ]
            }
            LoginStatus::FetchFailed(_) => p!["Failed to fetch login status"],
            LoginStatus::Fetching => p!["Loading"],
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
