use actix::prelude::*;
use actix_files::{Files, NamedFile};
use actix_multipart::{Multipart, MultipartError};
use actix_web::{get, post, web, App, HttpServer};
use std::fmt::Write;
use std::time;
use tokio_timer;

use shared;

mod count_actor;
use count_actor::{CountActor, MsgIncrement};

// ---- Apis ("/api/*") ----

#[post("send-message")]
fn send_message(
    state: web::Data<State>,
    request_data: web::Json<shared::SendMessageRequestBody>,
) -> impl Future<Item = web::Json<shared::SendMessageResponseBody>, Error = actix::MailboxError> {
    let text = request_data.text.clone();
    state
        .count_actor
        .send(MsgIncrement)
        .and_then(move |ordinal_number| {
            Ok(web::Json(shared::SendMessageResponseBody {
                ordinal_number,
                text,
            }))
        })
}

#[get("delayed-response/{delay}")]
fn delayed_response(
    delay: web::Path<(u64)>,
) -> impl Future<Item = String, Error = tokio_timer::Error> {
    tokio_timer::sleep(time::Duration::from_millis(*delay))
        .and_then(move |()| Ok(format!("Delay was set to {}ms.", delay)))
}

#[post("form")]
fn form(form: Multipart) -> impl Future<Item = String, Error = MultipartError> {
    form.map(|field| {
        // get field name
        let name = field
            .content_disposition()
            .and_then(|cd| cd.get_name().map(ToString::to_string))
            .expect("Can't get field name!");

        field
            // get field value stream
            .fold(Vec::new(), |mut value, bytes| -> Result<Vec<u8>, MultipartError> {
                for byte in bytes {
                    value.push(byte)
                }
                Ok(value)
            })
            .map(|value| String::from_utf8_lossy(&value).into_owned())
            // add name into stream
            .map(move |value| (name, value))
            .into_stream()
    })
    .flatten()
    .fold(
        String::new(),
        |mut output, (name, value)| -> Result<String, MultipartError> {
            writeln!(&mut output, "{}: {}", name, value).unwrap();
            writeln!(&mut output, "___________________").unwrap();
            Ok(output)
        },
    )
}

struct State {
    count_actor: Addr<CountActor>,
}

fn main() -> std::io::Result<()> {
    let system = System::new("server-integration-example");

    let count_actor_addr = CountActor(0).start();

    HttpServer::new(move || {
        App::new()
            .data(State {
                count_actor: count_actor_addr.clone(),
            })
            .service(
                web::scope("/api/")
                    .service(send_message)
                    .service(delayed_response)
                    .service(form)
                    .default_service(web::route().to(web::HttpResponse::NotFound)),
            )
            .service(Files::new("/public", "./client/public"))
            .service(Files::new("/pkg", "./client/pkg"))
            .default_service(web::get().to(|| NamedFile::open("./client/index.html")))
    })
    .bind("127.0.0.1:8000")?
    .run()?;

    system.run()
}
