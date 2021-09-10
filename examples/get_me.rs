use telegram_types::bot::methods::{GetMe, Method, TelegramResult};

async fn make_request<T: Method>(data: T) -> TelegramResult<T::Item> {
    let token = std::env::var("BOT_TOKEN").unwrap();
    let client = reqwest::Client::new();
    let res = client
        .post(T::url(&*token))
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
        .unwrap();
    let res = res.text().await.unwrap();
    println!("{:?}", res);
    serde_json::from_str(&res).unwrap()
}

#[tokio::main]
async fn main() {
    let get_me = make_request(GetMe).await;
    println!("{:?}", get_me);
}
