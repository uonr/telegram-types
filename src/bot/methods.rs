//! Request parameters types of Telegram bot methods.
use super::types;
use super::types::{ChatId, ForceReply, InlineKeyboardMarkup,
                   ParseMode, ReplyKeyboardMarkup, ReplyKeyboardRemove};


/// Chat integer identifier or username
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum ChatTarget {
    Id(ChatId),
    Username(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetUpdates {
    offset: Option<i32>,
    limit: Option<i32>,
    timeout: Option<i32>,
    allowed_updates: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetWebhook {
    url: String,
    // certificate
    max_connections: Option<i32>,
    allowed_updates: Option<Vec<String>>,
}


/// Kinds of reply markup.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

/// Send text messages. On success, the sent [`Message`](types::Message) is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendMessage {
    chat_id: ChatTarget,
    text: String,
    parse_mode: Option<ParseMode>,
    disable_web_page_preview: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<ReplyMarkup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForwardMessage {
    chat_id: ChatTarget,
    from_chat_id: ChatTarget,
    message_id: i32,
}

/// To get a list of profile pictures for a user. Returns a [`UserProfilePhotos`](types::UserProfilePhotos) object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetUserProfilePhotos {
    user_id: i32,
    offset: Option<i32>,
    limit: Option<i32>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChat {
    pub chat_id: ChatTarget,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMembersCount {
    pub chat_id: ChatTarget,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatAdministrators {
    pub chat_id: ChatTarget,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMember {
    pub chat_id: ChatTarget,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EditMessageText {
    chat_id: Option<ChatTarget>,
    message_id: Option<i32>,
    inline_message_id: Option<String>,
    text: String,
    parse_mode: Option<ParseMode>,
    disable_web_page_preview: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EditMessageCaption {
    chat_id: Option<ChatTarget>,
    message_id: Option<i32>,
    inline_message_id: Option<String>,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EditMessageReplyMarkup {
    chat_id: Option<ChatTarget>,
    message_id: Option<i32>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteMessage {
    chat_id: ChatTarget,
    message_id: i32,
}