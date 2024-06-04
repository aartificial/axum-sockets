use anyhow::Result;
use axum::routing::get;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Debug, Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(Debug, Serialize)]
struct MessageOut {
    text: String,
    user: String,
    date: DateTime<Utc>,
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {:?}", socket.id);

    socket.on("join", |socket: SocketRef, Data::<String>(room)| {
        info!("join received {:?}", room);
        let _ = socket.leave_all();
        let _ = socket.join(room);
    });

    socket.on("message", |socket: SocketRef, Data::<MessageIn>(data)| {
        info!("message received {:?}", data);

        let response = MessageOut {
            text: data.text,
            user: format!("anon-{}", socket.id),
            date: Utc::now(),
        };

        let _ = socket.within(data.room).emit("message", response);
    });
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);

    let router = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    info!("starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
