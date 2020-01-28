# tokio::process::ChildStdin cannot detect the termination of the child process on Linux?

See [Issue#2174](https://github.com/tokio-rs/tokio/issues/2174) in
tokio-rs/tokio for the current status.

This issue doesn't occur on macOS.  I haven't tested on Windows.

## Workaround

This issue should be solved in `tokio` basically, but I tried to find
workarounds to avoid this issue by waking up the pending write in some way.

* [workaround_oneshot.patch]
* [workaround_sigchld.patch]

These are NOT perfect solutions.

[workaround_oneshot.patch] can solve this issue even if we can call
`tx.send(())` explicitly at the correct timing.  That's impossible if we cannot
predict the child process termination.

[workaround_sigchld.patch] can solve this issue in the test program, but cannot
solve when there are multiple child processes.

[workaround_oneshot.patch]: ./workaround_oneshot.patch
[workaround_sigchld.patch]: ./workaround_sigchld.patch
