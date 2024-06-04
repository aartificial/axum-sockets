use anyhow::Result;
use axum::routing::get;
use socketioxide::extract::SocketRef;
use socketioxide::SocketIo;
use tracing::info;

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {:?}", socket.id);
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);

    let router = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    info!("starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
