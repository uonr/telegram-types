use reqwest::header::CONTENT_TYPE;
use std::fmt::Debug;
use telegram_types::bot::methods::{ChatTarget, GetUpdates, Method, SendDocument, TelegramResult};
use telegram_types::bot::types::{FileToSend, InputFile, Message, Update};

async fn make_request<T: Method + Debug>(data: &T) -> TelegramResult<T::Item> {
    let token = std::env::var("BOT_TOKEN").unwrap();
    let client = reqwest::Client::new();
    let res = client
        .post(T::url(&*token))
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(data).unwrap())
        .send()
        .await
        .unwrap();
    let res = res.text().await.unwrap();
    serde_json::from_str(&res).unwrap()
}

async fn upload(chat_id: ChatTarget<'_>) -> TelegramResult<Message> {
    let file_field = "document";
    let action = SendDocument::new(chat_id, FileToSend::InputFile(InputFile::new(file_field)));
    let token = std::env::var("BOT_TOKEN").unwrap();
    let client = reqwest::Client::new();
    let url = format!(
        "{}?{}",
        SendDocument::url(&*token),
        serde_urlencoded::to_string(action).unwrap()
    );
    let part = reqwest::multipart::Part::text("hello, world")
        .file_name("hello.txt")
        .mime_str("text/plain")
        .unwrap();
    let form = reqwest::multipart::Form::new().part(file_field, part);
    let res = client
        .post(url)
        .header(CONTENT_TYPE, "multipart/form-data")
        .multipart(form)
        .send()
        .await
        .unwrap();
    serde_json::from_slice(&*res.bytes().await.unwrap()).unwrap()
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
                        let chat_id = ChatTarget::Id(message.chat.id);
                        if text.contains("file") {
                            let sent = upload(chat_id).await;
                            dbg!(sent);
                        }
                    }
                }
                _ => {}
            }
            get_update.offset(update.update_id + 1);
        }
    }
}
