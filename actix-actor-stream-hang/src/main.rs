use std::time::{Duration, Instant};
use actix::prelude::*;

#[actix_rt::main]
async fn main() {
    let addr = MyActor::create(|ctx| {
        MyActor::add_stream(futures::stream::repeat(0u8), ctx);
        MyActor(Instant::now())
    });

    // Blocked at `addr.send(Ping).await.unwrap()` until `ctx.wait()` is called
    // in `StreamHandler::handle()`.
    addr.send(Ping).await.unwrap();

    System::current().stop();
    println!("Done");
}

struct MyActor(Instant);

impl Actor for MyActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Started");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("Stopped");
    }
}

struct Ping;

impl Message for Ping {
    type Result = ();
}

impl Handler<Ping> for MyActor {
    type Result = ();

    fn handle(&mut self, _: Ping, _: &mut Self::Context) -> Self::Result {
        println!("Ping");
    }
}

impl StreamHandler<u8> for MyActor {
    fn handle(&mut self, _: u8, ctx: &mut Context<Self>) {
        if self.0.elapsed() > Duration::from_secs(5) {
            // The context cannot be paused with
            // `ctx.wait(actix::fut::ready(()))`.
            ctx.wait(actix::fut::wrap_future(
                actix::clock::delay_for(Duration::from_secs(0))));
        }
    }
}
