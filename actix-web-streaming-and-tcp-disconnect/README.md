# actix-web streaming function and TCP disconnect

`actix-web` cannot detect TCP disconnect while streaming is pending
([Issue#1313](https://github.com/actix/actix-web/issues/1313)).

## Reproduction Environments

* macOS Catalina 10.15.2
  * Rust 1.40.0
* Linux (Docker Container)
  * [rust:1.40.0](https://hub.docker.com/_/rust)
* actix-web v2.0.0

### REST API

* `/pending`
  * `poll_next()` returns `Pending` forever
* `/alternate`
  * `poll_next()` returns `Pending` and `Ready` alternatively
* `/empty`
  * `poll_next()` returns an empty `Bytes` object.

## Reproduction Steps

### macOS

Build and run:

```console
cargo run
```

Open another terminal and run:

```
brew install coreutils
timeout 3 curl http://localhost:3000/pending
```

We never see the following message from `cargo run`:

```
PendingStream: Dropped
```

### Linux (VS Code Remote Containers)

Open this folder with VS Code Remote Containers.  And then press F5 to launch a
test server.

## Possible candidate of causes

One of the possible causes is that `actix-web` calls `write_xxx()` only when
receiving data from the Stream object. And calling `write_xxx()` is the only way
to detect the TCP disconnection in `tokio`.

The following implementation might solve this issue:

```rust
// This is a pseudo-code.

struct StreamingResponse<SO, ST> {
    socket: SO,
    stream: ST,
}

impl<SO, ST> Future for StreamingResponse<SO, ST>
where
    SO: AsyncWrite,
    ST: Stream,
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // Implement the following sequence with a state machine:
        //
        // (1) polling self.socket.poll_write() with an empty buffer
        //       in order to detect the TCP disconnection.
        // (2) polling self.stream.poll_next() in order to receive data
        // (3) polling self.socket.poll_write() in order to write the received
        //     data
        //
        // I'm not sure whether (1) is possible...
    }
```

## Test Results

| REST API | macOS         | Linux (Docker Container rust:1.4.0) |
|----------|---------------|-------------------------------------|
|/pending  | Reproduce     | Reproduce                           |
|/alternate| Not Reproduce | Reproduce (?)                       |
|/empty    | Reproduce     | Reproduce                           |