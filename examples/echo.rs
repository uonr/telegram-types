use telegram_types::bot::methods::{Method, TelegramResult, GetUpdates, SendMessage, ChatTarget};
use telegram_types::bot::types::Update;
use std::fmt::Debug;
use reqwest::header::CONTENT_TYPE;

async fn make_request<T: Method + Debug>(data: &T) -> TelegramResult<T::Item> {
    let token = std::env::var("BOT_TOKEN").unwrap();
    let client = reqwest::Client::new();
    let res = client.post(T::url(&*token))
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(data).unwrap())
        .send()
        .await
        .unwrap();
    let res = res.text().await.unwrap();
    serde_json::from_str(&res).unwrap()
}

#[tokio::main]
async fn main() {
    use telegram_types::bot::types::UpdateContent as Content;
    let mut get_update = GetUpdates::new();
    loop {
        let updates: Vec<Update> = make_request(&get_update).await.result.unwrap();
        for update in updates {
            // workaround for: https://github.com/serde-rs/serde/issues/1626
            let content = update.content.unwrap_or_default();
            match content {
                Content::Message(message) => {
                    if let Some(text) = message.text.as_ref() {
                        let send = SendMessage::new(ChatTarget::Id(message.chat.id), text);
                        make_request(&send).await;
                    }
                }
                Content::Poll(_) => {
                    println!("Poll");
                }
                Content::Unknown => {
                    println!("Unknown update")
                }
                _ => {}
            }
            get_update.offset(update.update_id + 1);
        }
    }
}