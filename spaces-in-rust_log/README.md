# tracing_subscriber::EnvFilter does not trim leading white spaces

## Reproduction Steps

`tracing_subscriber::EnvFilter`

```shell
$ cargo run
RUST_LOG='info , spaces_in_rust_log::x=debug'
Use tracing_subscriber::EnvFilter
ignoring ` spaces_in_rust_log::x=debug`: invalid filter directive
2023-01-28T04:30:29.890156Z  INFO spaces_in_rust_log:
2023-01-28T04:30:29.890175Z  INFO spaces_in_rust_log::x:
```

`env_logger`

```shell
$ cargo run -F env_logger
...
RUST_LOG='info , spaces_in_rust_log::x=debug'
Use env_logger
[2023-01-28T04:29:38Z INFO  spaces_in_rust_log]
[2023-01-28T04:29:38Z INFO  spaces_in_rust_log::x]
[2023-01-28T04:29:38Z DEBUG spaces_in_rust_log::x]
```

## Why?

`env_logger` trims white spaces before parsing each components:

* https://github.com/rust-cli/env_logger/blob/7e150d9c9fc45812a684aedff65bb5a6a0ad21a0/src/filter/mod.rs#L307

On the other hand, `tracing_subscriber::EnvFilter` doesn't trim white spaces:

* https://github.com/tokio-rs/tracing/blob/264a417b4b13728e7f46383b838668389ea8967e/tracing-subscriber/src/filter/env/builder.rs#L139-L141
* https://github.com/tokio-rs/tracing/blob/264a417b4b13728e7f46383b838668389ea8967e/tracing-subscriber/src/filter/env/builder.rs#L159-L161
