use seed::{prelude::*, *};
use web_sys::RequestCredentials;

const AUTH_SERVER: &str = "http://localhost:8081";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when our app starts.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    // Lets "order" our sign in status to be fetched.
    orders.send_msg(Msg::FetchIsSignedIn);
    Model::default()
}

// ------ ------
//     Model
// ------ ------

// We are only interested in storing the login status of our user so lets set the Model
// to a type alias.
type Model = Option<Result<bool, FetchError>>;

// ------ ------
//    Update
// ------ ------

enum Msg {
    FetchIsSignedIn,
    IsSignedInFetched(Result<bool, FetchError>),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchIsSignedIn => {
            // `perform_cmd` allows us to get a `Msg` from an `async` function.
            orders.perform_cmd(async { Msg::IsSignedInFetched(fetch_signed_in().await) });
            orders.skip();
        }
        // Once we have the data lets attach it to the model.
        Msg::IsSignedInFetched(data) => *model = Some(data),
    }
}

async fn fetch_signed_in() -> Result<bool, FetchError> {
    Request::new(&format!("{}/signed-in", AUTH_SERVER))
        // We have to allow cookies to be sent.
        .credentials(RequestCredentials::Include)
        .fetch()
        .await?
        .json()
        .await
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        p![
            "Use this link to toggle your login state. ",
            "Close the tab and come back. Note the state should be saved."
        ],
        match model {
            Some(Ok(true)) => {
                a![
                    "Sign Out",
                    attrs! {At::Href => format!("{}/sign-out",AUTH_SERVER)}
                ]
            }
            Some(Ok(false)) => {
                a![
                    "Sign In",
                    attrs! {At::Href => format!("{}/sign-in",AUTH_SERVER)}
                ]
            }
            Some(Err(_)) => p!["Failed to fetch login status"],
            None => p!["Loading"],
        }
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
