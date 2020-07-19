use std::io;

use actix_web::{App, HttpServer, HttpResponse, Responder};
use actix_web::web::Bytes;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(
        || {
            App::new()
                .service(streaming)
        })
        .bind("0.0.0.0:3000")?
        .workers(1)
        .run()
        .await
}

#[actix_web::get("/streaming")]
async fn streaming() -> impl Responder {
    let data: Result<Bytes, ()> = Ok(Bytes::from_static(b"hi"));
    HttpResponse::Ok()
        .set_header("content-type", "application/octet-stream")
        .streaming(futures::stream::repeat(data))
}
