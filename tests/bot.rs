extern crate serde;
extern crate serde_json;
extern crate telegram_types;

use serde_json::Value;
use telegram_types::bot::{methods, types};

pub fn from_result<T>(raw: &str) -> serde_json::Result<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let result: Value = serde_json::from_str(raw).unwrap();
    let value = result.get("result").unwrap().clone();
    serde_json::from_value::<T>(value)
}

#[test]
fn get_me() {
    from_result::<types::User>(&include_str!("json/getMe.json")).unwrap();
}

#[test]
fn empty_inline_keyboard_markup() {
    from_result::<types::InlineKeyboardMarkup>(&include_str!("json/empty.json")).unwrap();
}

#[test]
fn empty_reply_keyboard_markup() {
    from_result::<types::ReplyKeyboardMarkup>(&include_str!("json/empty.json")).unwrap();
}

#[test]
fn fake_inline_keyboard_markup() {
    let raw = &include_str!("json/fake_inline_keyboard_markup.json");
    let markup = from_result::<types::InlineKeyboardMarkup>(raw).unwrap();
    println!("{:?}", markup)
}

#[test]
fn fake_illegal_inline_keyboard_markup() {
    let raw = include_str!("json/fake_illegal_inline_keyboard_markup.json");
    let _markup = from_result::<types::InlineKeyboardMarkup>(&raw).unwrap();
}

#[test]
fn chat() {
    let raw = include_str!("json/chat.json");
    let _chat = serde_json::from_str::<types::Chat>(raw).unwrap();
}

#[test]
fn message() {
    let raw = include_str!("json/message.json");
    let _chat = serde_json::from_str::<types::Message>(raw).unwrap();
}

#[test]
fn update() {
    let raw = include_str!("json/update.json");
    let _updates = serde_json::from_str::<methods::UpdateList>(&raw).unwrap();
}

#[test]
fn failure() {
    let raw = include_str!("json/error.json");
    let update = serde_json::from_str::<methods::UpdateList>(&raw).unwrap();
    assert!(update.result.is_none());
}

#[test]
fn send_message() {
    let id = methods::ChatTarget::id(42);
    let params = methods::SendMessage::new(id, "hello, world");
    serde_json::to_string(&params).unwrap();
}
