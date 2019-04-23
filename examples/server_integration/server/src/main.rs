use actix_web::{get, HttpServer, App, Responder, web};
use actix_files::{Files, NamedFile};

use shared::Data;

#[get("/data")]
fn data_api() -> impl Responder {
    web::Json(
        Data {
            val: 7,
            text: "Test data".into(),
        }
    )
}

#[get("*")]
fn index() -> impl Responder {
    NamedFile::open("./client/index.html")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(data_api)
            .service(Files::new("/public", "./client/public"))
            .service(Files::new("/pkg", "./client/pkg"))
            .service(index)
    })
        .bind("127.0.0.1:8000")?
        .run()
}

