use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsgCreated {
    pub t: String,
    pub s: i64,
    pub op: i64,
    pub d: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub tts: bool,
    pub timestamp: String,
    pub referenced_message: Value,
    pub pinned: bool,
    pub nonce: String,
    pub mentions: Vec<Value>,
    pub mention_roles: Vec<Value>,
    pub mention_everyone: bool,
    pub member: Option<Member>,
    pub id: String,
    pub flags: i64,
    pub embeds: Vec<Value>,
    pub edited_timestamp: Value,
    pub content: String,
    pub components: Vec<Value>,
    pub channel_id: String,
    pub author: Author,
    pub attachments: Vec<Value>,
    pub guild_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub roles: Vec<String>,
    pub premium_since: Value,
    pub pending: bool,
    pub nick: Value,
    pub mute: bool,
    pub joined_at: String,
    pub flags: i64,
    pub deaf: bool,
    pub communication_disabled_until: Value,
    pub avatar: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub username: String,
    pub public_flags: i64,
    pub id: String,
    pub global_name: String,
    pub discriminator: String,
    pub avatar_decoration_data: Value,
    pub avatar: String,
}
