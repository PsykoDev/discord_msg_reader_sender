use reqwest::{Client, Response};

pub mod request;

pub async fn send_data(params: &request::Request, client: &Client, uri: String, token: &str) -> Result<Response, reqwest::Error> {
    params.send(&client, uri, token.to_string()).await

    // traitement de la Response ici ?
}
