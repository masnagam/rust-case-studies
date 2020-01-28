# tokio::process::ChildStdin cannot detect the termination of the child process on Linux?

See [Issue#2174](https://github.com/tokio-rs/tokio/issues/2174) in
tokio-rs/tokio for the current status.

This issue doesn't occur on macOS.  I haven't tested on Windows.

## Workaround

This issue should be solved in `tokio` basically.  However, there are
workarounds to avoid this issue by waking up the pending write in some way.  See
[workaround.patch](./workaround.patch) in this folder for details.  It might be
possible to apply a similar patch to `tokio::process::{ChildStdin, ChildStdout,
ChildStderr}`.
