#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;
use shared::Data;

#[get("/data", format = "application/json")]
fn data_api() -> String {
    let data = Data {
        val: 7,
        text: "Test data".into(),
    };

    serde_json::to_string(&data).unwrap()
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("./pkg"))
        .mount("/", routes![data_api])
        .launch();
}