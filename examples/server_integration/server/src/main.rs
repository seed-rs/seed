use actix_files::{Files, NamedFile};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use std::{thread, time};

// @TODO -- Once Actix 1.0 is stable and documentation is updated --
// @TODO: rewrite to actors (State and thread::sleep)
// @TODO: add Api handlers into one scope + better non-existent API handling
// @TODO: cannot use value 3000 as a 'delay' - a weird actix bug?

use shared;

type State = Arc<Mutex<StateData>>;

#[derive(Default)]
struct StateData {
    message_ordinal_number: u32,
}

#[post("/api/send-message")]
fn send_message(
    state: web::Data<State>,
    request_data: web::Json<shared::SendMessageRequestBody>,
) -> impl Responder {
    state.lock().unwrap().message_ordinal_number += 1;
    web::Json(shared::SendMessageResponseBody {
        ordinal_number: state.lock().unwrap().message_ordinal_number,
        text: request_data.text.clone(),
    })
}

#[get("/api/delayed-response/{delay}")]
fn delayed_response(delay: web::Path<(u64)>) -> impl Responder {
    thread::sleep(time::Duration::from_millis(*delay));
    format!("Delay was set to {}ms.", delay)
}

#[get("/api/*")]
fn non_existent_api() -> impl Responder {
    HttpResponse::NotFound()
}

#[get("*")]
fn index() -> impl Responder {
    NamedFile::open("./client/index.html")
}

fn main() -> std::io::Result<()> {
    let state = Arc::new(Mutex::new(StateData::default()));

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(send_message)
            .service(delayed_response)
            .service(non_existent_api)
            .service(Files::new("/public", "./client/public"))
            .service(Files::new("/pkg", "./client/pkg"))
            .service(index)
    })
    .bind("127.0.0.1:8000")?
    .run()
}
