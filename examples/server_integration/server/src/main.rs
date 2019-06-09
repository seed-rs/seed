use actix::prelude::*;
use actix_files::{Files, NamedFile};
use actix_web::{get, post, web, App, HttpServer};
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
                    .service(delayed_response),
            )
            .service(Files::new("/public", "./client/public"))
            .service(Files::new("/pkg", "./client/pkg"))
            .default_service(web::get().to(|| NamedFile::open("./client/index.html")))
    })
    .bind("127.0.0.1:8000")?
    .run()?;

    system.run()
}
