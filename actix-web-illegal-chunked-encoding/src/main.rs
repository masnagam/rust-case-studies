use std::io;
use std::pin::Pin;

use actix_service;
use actix_web::{App, HttpServer, HttpResponse, Responder};
use actix_web::web::Bytes;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(
        || {
            App::new()
                .wrap(RemoveUpgradeHeader)
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

struct RemoveUpgradeHeader;

impl<S, B> actix_service::Transform<S> for RemoveUpgradeHeader
where
    S: actix_service::Service<Request = actix_web::dev::ServiceRequest,
                              Response = actix_web::dev::ServiceResponse<B>,
                              Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = actix_web::dev::ServiceRequest;
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = RemoveUpgradeHeaderMiddleware<S>;
    type Future =
        futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(RemoveUpgradeHeaderMiddleware(service))
    }
}

struct RemoveUpgradeHeaderMiddleware<S>(S);

impl<S, B> actix_service::Service for RemoveUpgradeHeaderMiddleware<S>
where
    S: actix_service::Service<Request = actix_web::dev::ServiceRequest,
                              Response = actix_web::dev::ServiceResponse<B>,
                              Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = actix_web::dev::ServiceRequest;
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn futures::future::Future<
            Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, mut req: actix_web::dev::ServiceRequest) -> Self::Future {
        if req.headers().contains_key("upgrade") {
            req.headers_mut().remove("upgrade");
        }
        Box::pin(self.0.call(req))
    }
}
