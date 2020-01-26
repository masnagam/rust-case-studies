use std::process::Stdio;
use std::time::Duration;

use tokio::prelude::*;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut input = child.stdin.take().unwrap();

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
    println!("Waiting for the task...");
    let _ = handle.await;

    // However, the execution never reaches here unfortunately...
    println!("Done");
}
