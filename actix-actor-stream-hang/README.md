# actix-actor-stream-hang

`ActorStream` blocks other tasks in the same thread until `ctx.waiting()`
returns `true` or the `Stream` is pending.

* https://github.com/actix/actix/blob/master/src/stream.rs#L137

If the `Stream` can keep providing items for a long time, other tasks have no
chance to run unless `ctx.wait()` is called in `StreamHandler::handle()`.

* [src/main.rs#L13](./src/main.rs#L13)

The asynchronous task system used in Rust is some kind of
[cooperative multitasking system](https://en.wikipedia.org/wiki/Cooperative_multitasking).
So, I think that it might be better to provide a method to voluntarily yield
control.

A similar situation occurs in other types. For example:

* https://github.com/actix/actix/blob/master/src/mailbox.rs#L89

## Workaround

At this point, `ctx.wait(actix::fut::wrap_future(actix::clock::delay_for(Duration::from_secs(0))))`
is one of ways to pause the context and yield control.
