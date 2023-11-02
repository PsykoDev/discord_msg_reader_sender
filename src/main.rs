use std::{
    string::ToString,
    sync::Arc,
    time::Duration,
};

use futures_util::{SinkExt, StreamExt};
use reqwest;
use reqwest::Client;
use tokio;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

use crate::message_create::MsgCreated;
use crate::msg_response::MsgResponse;
use crate::payload::{Payload, PayloadGenerator};

mod payload;
mod msg_response;
mod data;
mod message_create;
mod utils;

const ME: &str = "<@204972632863539201>";
const CHANNEL_ID: i64 = 588028300690063469; //1062074714186592316

async fn heart_beat(socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>, hb: Duration) {
    let mut interval = tokio::time::interval(hb);
    loop {
        interval.tick().await;

        let mut socket = socket.lock().await;
        let message = Message::text(r#"{"op": 1, "d": null}"#);

        if let Err(err) = socket.send(message).await {
            error!("Error sending heartbeat: {:?}", err);
            break;
        }
    }
}

async fn payload(socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>, token: &str) {
    let msg = format!(r#"{{"op":2,"d":{{"token":"{token}","properties":{{"$os":"windows","$browser":"chrome","$device":"pc"}}}}}}"#);
    let mut socket = socket.lock().await;
    let payload = Message::text(msg);

    if let Err(err) = socket.send(payload).await {
        error!("Error sending payload: {:?}", err);
    }
    info!("Sent payload");
}

async fn receive_messages(socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>, tx: Sender<(String, String)>) {
    loop {
        let msg = socket.lock().await.next().await.unwrap();

        match msg {
            Ok(message) => {
                match serde_json::from_str::<MsgCreated>(message.to_text().unwrap()) {
                    Ok(msg) => {
                        if msg.d.guild_id.is_none() {
                            if msg.d.content.contains(ME) {
                                dm_ping!("[DM] {}: {}", msg.d.author.global_name, msg.d.content);
                            } else {
                                dm_msg!("[DM] {}: {}", msg.d.author.global_name, msg.d.content);
                            }
                        } else {
                            if msg.d.content.contains(ME) {
                                ping!("{}: {}", msg.d.author.global_name, msg.d.content);
                            } else {
                                msg!("{}: {}", msg.d.author.global_name, msg.d.content);
                                tx.send((msg.d.id, msg.d.channel_id)).await.unwrap();
                            }
                        }
                    }
                    Err(_) => {}
                }
                // println!("Received message: {:?}", message.to_string()); // print all msg
            }
            Err(err) => {
                error!("Error receiving message: {:?}", err);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let url = Url::parse("wss://gateway.discord.gg/?v=9&encording=json").unwrap();

    let msg_to_send = "Rust: LOVE";
    // token xhr
    let token = "";
    // emoji cat scream
    let emoji = "%F0%9F%99%80";

    let (socket, _) = connect_async(url).await.expect("can't connect");
    let socket = Arc::new(Mutex::new(socket));

    let hb_interval = Duration::from_millis(20000);

    let socket_clone = socket.clone();
    let (tx, mut rx) = mpsc::channel(16);
    info!("Send heart beat");
    tokio::spawn(heart_beat(socket_clone, hb_interval));
    info!("Send payload");
    tokio::spawn(payload(socket.clone(), token));
    info!("Receive messages in progress");
    tokio::spawn(receive_messages(socket.clone(), tx));


    let client = Client::new();


    loop {}

    let (msg_id, chan_id) = rx.recv().await.unwrap();

    // last msg id
    let mut last_message_id: i64 = msg_id.parse::<i64>().unwrap();

    // payload builder
    let payload = Payload::build_payload(format!("{}", msg_to_send));
    let serialized = serde_json::to_string(&payload).unwrap();

    let x = [
        data::request::Request::SendMsg { channel: CHANNEL_ID, payload: serialized },
        data::request::Request::SendReaction { channel: chan_id.parse::<i64>().unwrap(), message: last_message_id, emoji: emoji.to_string() },
    ];

    match data::send_data(&x.first().unwrap(), &client, "https://discord.com/api/v9/channels".to_string(), token).await {
        Ok(data) => {
            let x = serde_json::from_str::<MsgResponse>(&data.text().await.unwrap());
            //last_message_id = x.unwrap().id.parse::<i64>().unwrap();
        }
        Err(e) => println!("{}", e),
    }
}