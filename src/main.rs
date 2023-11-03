use std::{string::ToString, sync::Arc, time::Duration};
use std::collections::HashMap;

use futures_util::{SinkExt, StreamExt};
use rand::{RngCore, thread_rng};
use reqwest;
use reqwest::Client;
use tokio;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

use crate::message_create::MsgCreated;
use crate::payload::{Payload, PayloadGenerator};
use crate::ready::{Ready, Root};

mod payload;
mod msg_response;
mod data;
mod message_create;
mod utils;
mod ready;

const AUTOREPLY: bool = false;
const MSG_TO_SEND: &str = "Rust: :scream_cat:";
const TOKEN: &str = "";
const EMOJI: &str = "%F0%9F%99%80";
const ME: &str = "<@204972632863539201>";
const CHANNEL_ID: i64 = 588028300690063469; //1062074714186592316

/*
{"op":1,"d":0++} // hb
{"op":2,"d":{"token":"{token}","properties":{"$os":"windows","$browser":"chrome","$device":"pc"}}}
{"op":3,"d":{"status":"invisible","since":0,"activities":[],"afk":true}} // status
{"op":4,"d":{"guild_id":null,"channel_id":null,"self_mute":false,"self_deaf":false,"self_video":false,"flags":2}} // self action

{"op":8,"d":{"guild_id":["854214724169236480"],"user_ids":["830050566667239475","796868686970880000"],"presences":false}} // user_ids can see channel selected

{"op":13,"d":{"channel_id":"1084771054376992858"}} // change dm channel selection
{"op":14,"d":{"guild_id":"854214724169236480","typing":true,"threads":true,"activities":true,"members":[],"channels":{},"thread_member_lists":[]}} // typing info

*/

/*
PRESENCE_UPDATE

MESSAGE_UPDATE
MESSAGE_REACTION_ADD
MESSAGE_DELETE

CHANNEL_UNREAD_UPDATE
CHANNEL_TOPIC_UPDATE

VOICE_STATE_UPDATE
VOICE_CHANNEL_STATUS_UPDATE

SESSIONS_REPLACE
*/

async fn heart_beat(socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>, hb: Duration) {
    let mut interval = tokio::time::interval(hb);
    loop {
        interval.tick().await;

        let mut socket = socket.lock().await;
        let message = Message::text(format!(r#"{{"op": 1, "d": {}}}"#, thread_rng().next_u32()));

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
    let mut last_channel_id = 0;
    loop {
        let msg = socket.lock().await.next().await.unwrap().unwrap();
        let message = serde_json::from_str::<Ready>(&msg.to_text().unwrap());
        if let Ok(ref truc) = &message {
            match truc.t.to_lowercase().as_str() {
                "ready" => {
                    match serde_json::from_str::<Root>(&msg.to_text().unwrap()) {
                        Ok(e) => {
                            for x in e.d.guilds {
                                guild_id.insert(x.id, x.name);
                                for channel in x.channels {
                                    channel_id.insert(channel.id, channel.name);
                                }
                            }
                            println!("Server found {}", guild_id.len());
                            println!("Channel mapped {}", channel_id.len());
                        }
                        Err(e) => { error!("ready: {e}") }
                    }
                }

                "message_create" => {
                    match serde_json::from_str::<MsgCreated>(msg.to_text().unwrap()) {
                        Ok(msg) => {
                            if msg.d.guild_id.is_none() {
                                if msg.d.content.contains(ME) {
                                    dm_ping!("[DM] {}: {}", msg.d.author.global_name, msg.d.content);
                                } else {
                                    dm_msg!("[DM] {}: {}", msg.d.author.global_name, msg.d.content);
                                }
                            } else {
                                let c_id_parse = msg.d.channel_id.parse::<i64>().unwrap();
                                let content = msg.d.content;
                                if last_channel_id != c_id_parse {
                                    if content.contains(ME) {
                                        ping!("\n{} > {}\n\t{}: {}", guild_id.get(&msg.d.guild_id.unwrap()).unwrap(), channel_id.get(&msg.d.channel_id).unwrap(), format!("{}#{}", msg.d.author.username, msg.d.author.discriminator), content);
                                        last_channel_id = c_id_parse;
                                    } else {
                                        msg!("\n{} > {}\n\t{}: {}", guild_id.get(&msg.d.guild_id.unwrap()).unwrap(),channel_id.get(&msg.d.channel_id).unwrap(), format!("{}#{}", msg.d.author.username, msg.d.author.discriminator), content);
                                        last_channel_id = c_id_parse;
                                        tx.send((msg.d.id, msg.d.channel_id)).await.unwrap();
                                    }
                                } else {
                                    if content.contains(ME) {
                                        ping!("\t{}: {}", format!("{}#{}", msg.d.author.username, msg.d.author.discriminator), content);
                                        last_channel_id = c_id_parse;
                                    } else {
                                        msg!("\t{}: {}", format!("{}#{}", msg.d.author.username, msg.d.author.discriminator), content);
                                        last_channel_id = c_id_parse;
                                        tx.send((msg.d.id, msg.d.channel_id)).await.unwrap();
                                    }
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }

                "PRESENCE_UPDATE" => {}
                "MESSAGE_UPDATE" => {}
                "MESSAGE_REACTION_ADD" => {}
                "MESSAGE_DELETE" => {}
                "CHANNEL_UNREAD_UPDATE" => {}
                "CHANNEL_TOPIC_UPDATE" => {}
                "VOICE_STATE_UPDATE" => {}
                "VOICE_CHANNEL_STATUS_UPDATE" => {}
                "SESSIONS_REPLACE" => {}
                _ => {
                }
            }
        }
    }
}


#[tokio::main]
async fn main() {
    let url = Url::parse("wss://gateway.discord.gg/?encoding=json&v=9").unwrap();

    let (socket, _) = connect_async(url).await.expect("can't connect");
    let socket = Arc::new(Mutex::new(socket));

    let hb_interval = Duration::from_secs(30);

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

        let payload = Payload::build_payload(MSG_TO_SEND.to_string());
        let serialized = serde_json::to_string(&payload).unwrap();

        let x = [
            data::request::Request::SendMsg { channel: CHANNEL_ID, payload: serialized },
            data::request::Request::SendReaction { channel: chan_id.parse::<i64>().unwrap(), message: last_message_id, emoji: EMOJI.to_string() },
            data::request::Request::GetProfile { user_id: 000000000000 },
        ];

        match data::send_data(&x[0], &client, "https://discord.com/api/v9".to_string(), TOKEN).await {
            Ok(data) => {
                println!("{}", data.text().await.unwrap());
                //let _x = serde_json::from_str::<MsgResponse>(&data.text().await.unwrap());
                //last_message_id = x.unwrap().id.parse::<i64>().unwrap();
            }
            Err(e) => println!("{}", e),
        }
    }
}