use reqwest::{Client, Response};

pub enum Request {
    SendMsg { channel: i64, payload: String },
    SendReaction { channel: i64, message: i64, emoji: String },
    GetProfile {user_id: i64}
}

impl Request {
    pub async fn send(&self, client: &Client, base_uri: String, token: String) -> Result<Response, reqwest::Error> {
        let (req_builder, accept) = match self {
            Request::SendMsg { channel, payload, .. } => (client.post(format!("{base_uri}/channels/{channel}/messages")).body(payload.to_string()), "*/*"),
            Request::SendReaction { channel, message, emoji, .. } => (client.put(format!("{base_uri}/channels/{channel}/messages/{message}/reactions/{emoji}/%40me?location=Message&type=0")).header("Content-Length", 0), "text/html; charset=utf-8"),
            Request::GetProfile {user_id} => (client.get(format!("{base_uri}/users/{user_id}/profile")), "*/*"),
        };

        let req_prepared = req_builder
            .header("authorization", token)
            .header("accept", accept)
            .header("content-type", "application/json")
            .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36");

        req_prepared.send().await
    }
}