use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    serve, Router,
};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::sync::mpsc;

async fn task_a(mut rx_a: mpsc::UnboundedReceiver<String>, tx_b: mpsc::UnboundedSender<String>) {
    while let Some(msg) = rx_a.recv().await {
        let _ = tx_b.send(msg.clone());
        let _ = tx_b.send(msg);
    }
}

async fn task_b(
    mut rx_b: mpsc::UnboundedReceiver<String>,
    mut tx_ws: futures::stream::SplitSink<WebSocket, Message>,
) {
    while let Some(msg) = rx_b.recv().await {
        if tx_ws.send(Message::Text(msg.into())).await.is_err() {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(ws_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(ws: WebSocket) {
    let (tx_a, rx_a) = mpsc::unbounded_channel::<String>();
    let (tx_b, rx_b) = mpsc::unbounded_channel::<String>();

    let (tx_ws, mut rx_ws) = ws.split();

    tokio::spawn(task_a(rx_a, tx_b.clone()));
    tokio::spawn(task_b(rx_b, tx_ws));

    while let Some(Ok(message)) = rx_ws.next().await {
        if let Message::Text(text) = message {
            if tx_a.send(text.to_string()).is_err() {
                break;
            }
        }
    }
}
