use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::{App, HttpServer, HttpResponse};
use actix_web::web::Bytes;
use futures::stream::Stream;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(
        || {
            App::new()
                .service(pending)
                .service(alternate)
                .service(empty)
        })
        .bind("0.0.0.0:3000")?
        .workers(1)
        .run()
        .await
}

#[actix_web::get("/pending")]
async fn pending() -> io::Result<HttpResponse> {
    Ok(HttpResponse::Ok().streaming(PendingStream))
}

struct PendingStream;

impl Stream for PendingStream {
    type Item = io::Result<Bytes>;

    fn poll_next(
        self: Pin<&mut Self>,
        _: &mut Context
    ) -> Poll<Option<Self::Item>> {
        eprintln!("PendingStream: Pending");
        Poll::Pending
    }
}

impl Drop for PendingStream {
    fn drop(&mut self) {
        eprintln!("PendingStream: Dropped");
    }
}

#[actix_web::get("/alternate")]
async fn alternate() -> io::Result<HttpResponse> {
    Ok(HttpResponse::Ok().streaming(AlternateStream(false)))
}

struct AlternateStream(bool);

impl Stream for AlternateStream {
    type Item = io::Result<Bytes>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _: &mut Context
    ) -> Poll<Option<Self::Item>> {
        if self.0 {
            eprintln!("AlternateStream: Ready");
            self.0 = false;
            Poll::Ready(Some(Ok(Bytes::from("0"))))
        } else {
            eprintln!("AlternateStream: Pending");
            self.0 = true;
            Poll::Pending
        }
    }
}

impl Drop for AlternateStream {
    fn drop(&mut self) {
        eprintln!("AlternateStream: Dropped");
    }
}

#[actix_web::get("/empty")]
async fn empty() -> io::Result<HttpResponse> {
    Ok(HttpResponse::Ok().streaming(EmptyStream))
}

struct EmptyStream;

impl Stream for EmptyStream {
    type Item = io::Result<Bytes>;

    fn poll_next(
        self: Pin<&mut Self>,
        _: &mut Context
    ) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(Ok(Bytes::from(""))))
    }
}

impl Drop for EmptyStream {
    fn drop(&mut self) {
        eprintln!("EmptyStream: Dropped");
    }
}
