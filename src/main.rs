use std::{string::ToString, sync::Arc, time::Duration};
use std::collections::HashMap;

use futures_util::{SinkExt, StreamExt};
use reqwest;
use reqwest::Client;
use tokio;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

use crate::message_create::MsgCreated;
use crate::msg_response::MsgResponse;
use crate::payload::{Payload, PayloadGenerator};
use crate::ready::Root;

mod payload;
mod msg_response;
mod data;
mod message_create;
mod utils;
mod ready;

const AUTOREPLY: bool = false;
const MSG_TO_SEND: &str = "Rust: LOVE";
const TOKEN: &str = "";
const EMOJI: &str = "%F0%9F%99%80";
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
    let mut guild_id = HashMap::new();
    let mut channel_id = HashMap::new();
    loop {
        let msg = socket.lock().await.next().await.unwrap();

        match msg {
            Ok(message) => {
                match serde_json::from_str::<Root>(message.to_text().unwrap()) {
                    Ok(msg) => {
                        for x in msg.d.guilds {
                            guild_id.insert(x.id, x.name);
                            for channel in x.channels {
                                channel_id.insert(channel.id, channel.name);
                            }
                        }
                        println!("Server found {}", guild_id.len());
                        println!("Channel mapped {}", channel_id.len());
                    }
                    Err(_) => {
                        //println!("error {e}");
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
                                        ping!("{} > {}\n\t{}: {}", guild_id.get(&msg.d.guild_id.unwrap()).unwrap(),channel_id.get(&msg.d.channel_id).unwrap(), msg.d.author.global_name, msg.d.content);
                                    } else {
                                        msg!("{} > {}\n\t{}: {}", guild_id.get(&msg.d.guild_id.unwrap()).unwrap(),channel_id.get(&msg.d.channel_id).unwrap(), msg.d.author.global_name, msg.d.content);
                                        tx.send((msg.d.id, msg.d.channel_id)).await.unwrap();
                                    }
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
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

    let (socket, _) = connect_async(url).await.expect("can't connect");
    let socket = Arc::new(Mutex::new(socket));

    let hb_interval = Duration::from_millis(15000);

    let socket_clone = socket.clone();
    let mut thread: Vec<JoinHandle<()>> = Vec::new();
    let (tx, mut rx) = mpsc::channel(16);
    info!("Send heart beat");
    thread.push(tokio::spawn(heart_beat(socket_clone, hb_interval)));
    info!("Send payload");
    thread.push(tokio::spawn(payload(socket.clone(), TOKEN)));
    info!("Receive messages in progress");
    thread.push(tokio::spawn(receive_messages(socket.clone(), tx)));

    let _ = futures::future::join_all(thread).await;

    let client = Client::new();

    while AUTOREPLY {
        let (msg_id, chan_id) = rx.recv().await.unwrap();

        let last_message_id: i64 = msg_id.parse::<i64>().unwrap();

        let payload = Payload::build_payload(format!("{}", MSG_TO_SEND));
        let serialized = serde_json::to_string(&payload).unwrap();

        let x = [
            data::request::Request::SendMsg { channel: CHANNEL_ID, payload: serialized },
            data::request::Request::SendReaction { channel: chan_id.parse::<i64>().unwrap(), message: last_message_id, emoji: EMOJI.to_string() },
        ];

        match data::send_data(&x.first().unwrap(), &client, "https://discord.com/api/v9/channels".to_string(), TOKEN).await {
            Ok(data) => {
                let _x = serde_json::from_str::<MsgResponse>(&data.text().await.unwrap());
                //last_message_id = x.unwrap().id.parse::<i64>().unwrap();
            }
            Err(e) => println!("{}", e),
        }
    }
}