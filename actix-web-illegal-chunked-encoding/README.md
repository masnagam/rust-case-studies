# actix-web illegal chunked-encoding issue

When `actix-web` receives an HTTP request with `Upgrade: h2c` header for a
resource which will be encoded with the chunked-encoding, it responds an illegal
chunked-encoding response.

## Reproduction Environments

* macOS Catalina 10.15.5
  * Rust 1.45.0
* Linux (Docker Container)
  * [rust:1.45.0](https://hub.docker.com/_/rust)
* actix-web v2.0.0
  * The same issue still occurs with actix 3.0.0-beta.1

## Reproduction Steps

### macOS

Build and run:

```console
cargo run
```

Open another terminal and run:

```
curl -v -H "Upgrade: h2c" http://localhost:3000/streaming >/dev/null
```

Then you can see the following error message:

```
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0*   Trying 127.0.0.1...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 3000 (#0)
> GET /streaming HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.64.1
> Accept: */*
> Upgrade: h2c
>
< HTTP/1.1 200 OK
< transfer-encoding: chunked
< content-type: application/octet-stream
< date: Sun, 19 Jul 2020 08:11:24 GMT
<
{ [32644 bytes data]
* Illegal or missing hexadecimal sequence in chunked-encoding
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
* Closing connection 0
curl: (56) Illegal or missing hexadecimal sequence in chunked-encoding
```

As described the message above, there was no hexadecimal sequence in
chunked-encoding even though the HTTP version of the response is 1.1.

### Linux (VS Code Remote Containers)

1. Open this folder with VS Code Remote Containers
2. Press F5 to launch a test server
3. Run the same `curl` command on the `bash` terminal on VSCode

## Workaround

Add the following lines in your `Cargo.toml`:

```toml
[patch.crates-io]
actix-http = { git = "https://github.com/masnagam/actix-web", branch = "fix-1611" }
```

This will replace the original actix-http with one in masnagam/actix-web.

As you can see in src/main.rs, this issue cannot be solved by using a middleware
which removes the `Upgrade` header.  Because `actix-http` processes the
`Upgrade` header before the middleware processes it.
