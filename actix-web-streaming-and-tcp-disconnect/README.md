# actix-web streaming function and TCP disconnect

`actix-web` cannot detect TCP disconnect while streaming is pending
([Issue#1313](https://github.com/actix/actix-web/issues/1313)).

## Conclusion

I concluded that it's difficult to detect client disconnects while streams are
pending.

tokio's TcpStream seems to be based on Rust's TcpStream which has a traditional
socket interface.  TcpStream's read/write function call with an empty buffer
returns `Ok(0)`.  So, it's impossible to distinguish between client disconnect
and success.

[Issue#29](https://github.com/tokio-rs/tokio/issues/29) in the tokio project is
a similar topic.

I checked behavior of [tokio::io::PollEvented::poll_write_ready()] on Linux.  I
analyzed my test program by using VS Code debugger and found that this function
doesn't detect client disconnect.  [tokio::net::TcpStream::poll_write_priv()]
reached `mio::net::TcpStream::write()` which detected the client disconnect.

Read/Write with non-empty buffer can detect the client disconnect.  But that
breaks the HTTP request pipelining and the HTTP response.

There might be a platform specific workaround, but there is no useful functions
in crates that actix-web uses at this moment.

Finally, I decided to feed some data at short intervals from my stream
implementation in order to detect client disconnect quickly.

[tokio::io::PollEvented::poll_write_ready()]: https://github.com/tokio-rs/tokio/blob/master/tokio/src/io/poll_evented.rs#L302
[tokio::net::TcpStream::poll_write_priv()]: https://github.com/tokio-rs/tokio/blob/master/tokio/src/net/tcp/stream.rs#L668

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
