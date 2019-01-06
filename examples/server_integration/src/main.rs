#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
use rocket::http::Method;
use rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use serde::{Serialize, Deserialize};
use serde_json;

use shared::Data;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/data", format = "application/json")]
fn data_api() -> String {
    let data = Data {
        val: 7,
        text: "Test data".into(),
    };

    serde_json::to_string(&data).unwrap()
}

fn main() {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:8000"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", routes![index, data_api])
        .attach(options)
        .launch();
}