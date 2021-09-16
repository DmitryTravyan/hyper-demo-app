use tokio::signal::unix::SignalKind;

pub async fn shutdown_signal() {
    warn!("test");
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}