use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    content: String,
    nonce: i32,
    tts: bool,
}

impl PayloadGenerator for Payload {
    fn build_payload(content: String) -> Payload {
        Payload {
            content,
            nonce: nonce_generator(),
            tts: false,
        }
    }
}

pub trait PayloadGenerator {
    fn build_payload(content: String) -> Payload;
}

fn nonce_generator() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}