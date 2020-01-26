use std::pin::Pin;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::prelude::*;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .unwrap();

    let writing = Arc::new(Mutex::new(false));
    let shutdown = Arc::new(Mutex::new(false));

    let mut input = Wrapper {
        inner: child.stdin.take().unwrap(),
        writing: writing.clone(),
        shutdown: shutdown.clone(),
    };

    // Spawn a task which writes large amounts of data to `input`.
    let handle = tokio::spawn(async move {
        let data = [0u8; 8192];
        loop {
            if let Err(err) = input.write_all(&data).await {
                println!("Input error: {}", err);
                return;
            }
        }
    });

    // Wait 1 second so that the task is blocked at `input.write_all()`.
    tokio::time::delay_for(Duration::from_secs(1)).await;

    // Kill the child process while the task is blocked at `input.write_all()`.
    let result = child.kill();
    assert!(result.is_ok());
    let _ = child.await;

    // Expected behavior
    // -----------------
    // The following `await` returns quickly because the task ends with a broken
    // pipe error.
    println!("Waiting for the task...: writing({}) shutdown({})",
             *writing.lock().unwrap(), *shutdown.lock().unwrap());
    let _ = handle.await;

    // However, the execution never reaches here unfortunately...
    println!("Done");
}

struct Wrapper<W> {
    inner: W,
    writing: Arc<Mutex<bool>>,
    shutdown: Arc<Mutex<bool>>,
}

impl<W: AsyncWrite + Unpin> AsyncWrite for Wrapper<W> {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8]
    ) -> Poll<io::Result<usize>> {
        let poll = Pin::new(&mut self.inner).poll_write(cx, buf);
        *self.writing.lock().unwrap() = poll.is_pending();
        poll
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    #[inline]
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context
    ) -> Poll<io::Result<()>> {
        *self.shutdown.lock().unwrap() = true;
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}
