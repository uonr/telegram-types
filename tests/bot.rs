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
fn update_my_chat_member() {
    let raw = include_str!("json/update_my_chat_member.json");
    let _updates = serde_json::from_str::<methods::UpdateList>(&raw).unwrap();
}

#[test]
fn failure() {
    let raw = include_str!("json/error.json");
    let update = serde_json::from_str::<methods::UpdateList>(&raw).unwrap();
    assert!(update.result.is_none());
    assert_eq!(update.error_code, Some(401));
    assert_eq!(update.description, Some("Unauthorized".to_string()))
}

#[test]
fn send_message() {
    let id = methods::ChatTarget::id(42);
    let params = methods::SendMessage::new(id, "hello, world");
    serde_json::to_string(&params).unwrap();
}

#[test]
fn unknown() {
    use serde_json::from_str;
    use types::{ChatType, MessageEntityKind};
    let raw = "\"Flip Flappers\"";
    let message_entity_type = from_str::<MessageEntityKind>(raw).unwrap();
    assert_eq!(message_entity_type, MessageEntityKind::Unknown);
    let raw = r#"{"type": "Papika", "Cocona": "Mimi"}"#;
    let chat_type = from_str::<ChatType>(raw).unwrap();
    assert_eq!(chat_type, ChatType::Unknown);
}

#[test]
fn file_id() {
    use serde_json::{from_str, to_string};
    use telegram_types::bot::types::FileToSend;
    let file_id = types::FileId("42".to_string());
    let file_to_send = FileToSend::FileId(file_id);
    let file_id_serialized = to_string(&file_to_send).unwrap();
    let file_id_deserialized = from_str::<FileToSend>(&*file_id_serialized);
    assert_eq!(file_id_serialized, "\"42\"".to_string());
    assert_eq!(file_id_deserialized.unwrap(), file_to_send);
}

#[test]
fn input_file() {
    use serde_json::to_string;
    use telegram_types::bot::types::{FileToSend, InputFile};
    let input_file = InputFile::new("cocona.webp");
    let input_file = FileToSend::InputFile(input_file);
    let input_file_serialized = to_string(&input_file).unwrap();
    assert_eq!(input_file_serialized, r#""attach://cocona.webp""#);
}
