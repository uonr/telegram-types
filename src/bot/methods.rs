//! Request parameters types of Telegram bot methods.
use std::default::Default;
use super::types;
use super::types::{ChatId, ForceReply, InlineKeyboardMarkup, MessageId, ParseMode, ReplyKeyboardMarkup,
                   ReplyKeyboardRemove, UpdateId, UserId};


/// Chat integer identifier or username
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum ChatTarget {
    Id(ChatId),
    Username(String),
}

/// Use this method to receive incoming updates using long
/// polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)).
/// An Array of [`Update`] objects is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GetUpdates {
    pub offset: Option<UpdateId>,
    pub limit: Option<i32>,
    pub timeout: Option<i32>,
    pub allowed_updates: Option<Vec<String>>,
}


impl GetUpdates {
    pub fn new() -> GetUpdates {
        Default::default()
    }

    pub fn offset(&mut self, x: UpdateId) {
        self.offset = Some(x)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct UpdateList (pub Vec<types::Update>);

/// Use this method to specify a url and receive incoming updates via an outgoing webhook.
/// Whenever there is an update for the bot, we will send an HTTPS POST request to the specified
/// url, containing a JSON-serialized [`Update`]. In case of an unsuccessful request, we will give up
/// after a reasonable amount of attempts. Returns True on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Eq, Ord, Hash)]
pub struct SetWebhook {
    pub url: String,
    // certificate
    pub ax_connections: Option<i32>,
    pub allowed_updates: Option<Vec<String>>,
}


impl SetWebhook {
    pub fn new(url: String) -> SetWebhook {
        SetWebhook {
            url,
            ax_connections: None,
            allowed_updates: None,
        }
    }
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
    pub chat_id: ChatTarget,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<MessageId>,
    pub reply_markup: Option<ReplyMarkup>,
}


impl SendMessage {
    pub fn new(chat_id: ChatTarget, text: String) -> SendMessage {
        SendMessage {
            chat_id,
            text,
            parse_mode: None,
            disable_web_page_preview: Some(false),
            reply_to_message_id: None,
            disable_notification: Some(false),
            reply_markup: None,
        }
    }

    pub fn reply(chat_id: ChatTarget, text: String, message_id: MessageId) -> SendMessage {
        let message = Self::new(chat_id, text);
        SendMessage {
            reply_to_message_id: Some(message_id),
            ..message
        }
    }
}


/// Use this method to forward messages of any kind. On success, the sent `Message` is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForwardMessage {
    pub chat_id: ChatTarget,
    pub from_chat_id: ChatTarget,
    pub message_id: MessageId,
}

/// To get a list of profile pictures for a user. Returns a [`UserProfilePhotos`](types::UserProfilePhotos) object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetUserProfilePhotos {
    pub user_id: UserId,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}


/// Use this method to get up to date information about the chat (current name of the user
/// for one-on-one conversations, current username of a user, group or channel, etc.).
///
/// Returns a [`Chat`] object on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChat {
    pub chat_id: ChatTarget,
}

/// Use this method to get the number of members in a chat. Returns `Int` on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMembersCount {
    pub chat_id: ChatTarget,
}

/// Use this method to get a list of administrators in a chat. On success, returns an Array
/// of `ChatMember` objects that contains information about all chat administrators except
/// other bots. If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatAdministrators {
    pub chat_id: ChatTarget,
}

/// Use this method to get information about a member of a chat. Returns a `ChatMember`
/// object on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMember {
    pub chat_id: ChatTarget,
    pub user_id: UserId,
}


/// Use this method to edit text and game messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`] is returned,
/// otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EditMessageText {
    pub chat_id: Option<ChatTarget>,
    pub message_id: Option<MessageId>,
    pub inline_message_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    pub disable_web_page_preview: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit captions of messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`] is returned,
/// otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EditMessageCaption {
    pub chat_id: Option<ChatTarget>,
    pub message_id: Option<MessageId>,
    pub inline_message_id: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


/// Use this method to edit only the reply markup of messages sent by the bot or via the bot (for
/// inline bots). On success, if edited message is sent by the bot, the edited [`Message`] is returned,
/// otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EditMessageReplyMarkup {
    pub chat_id: Option<ChatTarget>,
    pub message_id: Option<MessageId>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


/// Use this method to delete a message, including service messages, with the following limitations:
///
/// - A message can only be deleted if it was sent less than 48 hours ago.
/// - Bots can delete outgoing messages in groups and supergroups.
/// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
/// - If the bot is an administrator of a group, it can delete any message there.
/// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
///
/// Returns True on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteMessage {
    pub chat_id: ChatTarget,
    pub message_id: MessageId,
}