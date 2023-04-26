use tokio::signal::*;

#[cfg(unix)]
pub async fn termination() {
    let mut term_stream =
        unix::signal(unix::SignalKind::terminate()).expect("failed to handle signal SIGTERM");
    let mut int_stream =
        unix::signal(unix::SignalKind::interrupt()).expect("failed to handle signal SIGINT");

    tokio::select! {
        _ = term_stream.recv() => {},
        _ = int_stream.recv() => {},
    };
}

#[cfg(windows)]
pub async fn termination() {
    let mut win_ctrl_c = windows::ctrl_c().expect("failed to handle signal");
    win_ctrl_c.recv().await;
}
