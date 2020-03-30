# actix-started-in-drop

> actix/actix/issues/372

## Description

`Actor::started()` may be called in the finalization.  In this case, using the
context in `Actor::started()` panics.

This issue occurs at least on Linux and macOS.

This issue also occurs in the latest master branch.

## Workaround

A possible workaround is yielding the task at least once before the finalization
like below:

```rust
#[actix_rt::main]
async fn main() {
    let _addr = MyActor.start();

    tokio::task::yield_now().await;
}
```
