use std::time::Duration;

use actix::prelude::*;

#[actix_rt::main]
async fn main() {
    let _addr = MyActor.start();
}

struct MyActor;

impl MyActor {
    fn noop(&mut self, _: &mut Context<Self>) {}
}

impl Actor for MyActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_later(Duration::from_secs(1), Self::noop);
    }
}
