//! Request parameters types of Telegram bot methods.
use super::types;
use super::types::{
    ChatId, FileToSend, ForceReply, InlineKeyboardMarkup, MessageId, ParseMode,
    ReplyKeyboardMarkup, ReplyKeyboardRemove, UpdateId, UserId,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;
use std::default::Default;
use std::error::Error;
use std::fmt;
use super::types::InputMedia;
use std::net::IpAddr;

/// Chat integer identifier or username
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum ChatTarget<'a> {
    Id(ChatId),
    Username(Cow<'a, str>),
}

impl<'a> ChatTarget<'a> {
    pub fn id(value: i64) -> ChatTarget<'a> {
        ChatTarget::Id(ChatId(value))
    }

    pub fn username<T: Into<Cow<'a, str>>>(name: T) -> ChatTarget<'a> {
        ChatTarget::Username(name.into())
    }
}

/// Use this method to receive incoming updates using long
/// polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)).
/// An Array of [`Update`](types::Update) objects is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct GetUpdates<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<UpdateId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Cow<'a, [UpdateTypes]>>,
}

impl<'a> GetUpdates<'a> {
    pub fn new() -> GetUpdates<'a> {
        Default::default()
    }

    pub fn offset(&mut self, x: UpdateId) {
        self.offset = Some(x)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ApiError {
    pub error_code: i32,
    pub description: String,
    pub parameters: Option<types::ResponseParameters>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ERROR] {}", self.description)
    }
}

impl Error for ApiError {
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}

/// Use this method to specify a url and receive incoming updates via an outgoing webhook.
/// Whenever there is an update for the bot, we will send an HTTPS POST request to the specified
/// url, containing a JSON-serialized [`Update`](types::Update). In case of an unsuccessful request, we will give up
/// after a reasonable amount of attempts. Returns True on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetWebhook<'a> {
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: Cow<'a, str>,

    /// The fixed IP address which will be used to send webhook requests instead of the IP address
    /// resolved through DNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<IpAddr>,

    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery,
    /// 1-100. Defaults to *40*. Use lower values to limit the load on your bot's server, and higher
    /// values to increase your bot's throughput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Cow<'a, [UpdateTypes]>>,

    /// Pass True to drop all pending updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

impl<'a> SetWebhook<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(url: T) -> SetWebhook<'a> {
        SetWebhook {
            url: url.into(),
            max_connections: None,
            allowed_updates: None,
            ip_address: None,
            drop_pending_updates: None,
        }
    }

    pub fn max_connections(self, x: i32) -> SetWebhook<'a> {
        SetWebhook {
            max_connections: Some(x),
            ..self
        }
    }
}

/// Kinds of reply markup.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

/// Send text messages. On success, the sent [`Message`](types::Message) is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendMessage<'a> {
    pub chat_id: ChatTarget<'a>,
    pub text: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'a> SendMessage<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(chat_id: ChatTarget<'a>, text: T) -> SendMessage<'a> {
        SendMessage {
            chat_id,
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: Some(false),
            reply_to_message_id: None,
            disable_notification: Some(false),
            reply_markup: None,
        }
    }

    pub fn parse_mode(self, mode: ParseMode) -> SendMessage<'a> {
        SendMessage {
            parse_mode: Some(mode),
            ..self
        }
    }

    pub fn reply(self, message_id: MessageId) -> SendMessage<'a> {
        SendMessage {
            reply_to_message_id: Some(message_id),
            ..self
        }
    }

    pub fn reply_markup(self, markup: ReplyMarkup) -> Self {
        Self {
            reply_markup: Some(markup),
            ..self
        }
    }
}

/// Use this method to send .webp stickers.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendSticker<'a> {
    pub chat_id: ChatTarget<'a>,
    pub sticker: FileToSend,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'a> SendSticker<'a> {
    pub fn new(chat_id: ChatTarget<'a>, sticker: FileToSend) -> SendSticker<'a> {
        SendSticker {
            chat_id,
            sticker,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn reply(self, reply_to_message_id: MessageId) -> SendSticker<'a> {
        SendSticker {
            reply_to_message_id: Some(reply_to_message_id),
            ..self
        }
    }

    pub fn reply_markup(self, markup: ReplyMarkup) -> Self {
        Self {
            reply_markup: Some(markup),
            ..self
        }
    }
}

/// Use this method to send photos.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendPhoto<'a> {
    pub chat_id: ChatTarget<'a>,
    pub photo: FileToSend,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'a> SendPhoto<'a> {
    pub fn new(chat_id: ChatTarget<'a>, photo: FileToSend) -> SendPhoto<'a> {
        SendPhoto {
            chat_id,
            photo,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn parse_mode(self, mode: ParseMode) -> SendPhoto<'a> {
        SendPhoto {
            parse_mode: Some(mode),
            ..self
        }
    }

    pub fn reply(self, reply_to_message_id: MessageId) -> SendPhoto<'a> {
        SendPhoto {
            reply_to_message_id: Some(reply_to_message_id),
            ..self
        }
    }

    pub fn reply_markup(self, markup: ReplyMarkup) -> Self {
        Self {
            reply_markup: Some(markup),
            ..self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendDocument<'a> {
    pub chat_id: ChatTarget<'a>,
    pub document: FileToSend,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'a> SendDocument<'a> {
    pub fn new(chat_id: ChatTarget<'a>, document: FileToSend) -> SendDocument<'a> {
        SendDocument {
            chat_id,
            document,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn parse_mode(self, mode: ParseMode) -> SendDocument<'a> {
        SendDocument {
            parse_mode: Some(mode),
            ..self
        }
    }

    pub fn reply(self, reply_to_message_id: MessageId) -> SendDocument<'a> {
        SendDocument {
            reply_to_message_id: Some(reply_to_message_id),
            ..self
        }
    }

    pub fn reply_markup(self, markup: ReplyMarkup) -> Self {
        Self {
            reply_markup: Some(markup),
            ..self
        }
    }
}

/// Use this method to forward messages of any kind.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForwardMessage<'a> {
    pub chat_id: ChatTarget<'a>,
    pub from_chat_id: ChatTarget<'a>,
    pub message_id: MessageId,
}

/// To get a list of profile pictures for a user. Returns a [`UserProfilePhotos`](types::UserProfilePhotos) object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetUserProfilePhotos {
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl GetUserProfilePhotos {
    pub fn new(user_id: UserId) -> GetUserProfilePhotos {
        GetUserProfilePhotos {
            user_id,
            offset: None,
            limit: None,
        }
    }
}

/// Use this method to get up to date information about the chat (current name of the user
/// for one-on-one conversations, current username of a user, group or channel, etc.).
///
/// Returns a [`Chat`](types::Chat) object on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChat<'a> {
    pub chat_id: ChatTarget<'a>,
}

/// Use this method to get the number of members in a chat. Returns `Int` on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChatMembersCount<'a> {
    pub chat_id: ChatTarget<'a>,
}

/// Use this method to get a list of administrators in a chat. On success, returns an Array
/// of `ChatMember` objects that contains information about all chat administrators except
/// other bots. If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChatAdministrators<'a> {
    pub chat_id: ChatTarget<'a>,
}

/// Use this method to get information about a member of a chat. Returns a `ChatMember`
/// object on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChatMember<'a> {
    pub chat_id: ChatTarget<'a>,
    pub user_id: UserId,
}

/// Use this method to edit text and game messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`](types::Message) is
/// returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct EditMessageText<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatTarget<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Cow<'a, str>>,
    pub text: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> EditMessageText<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(
        chat_id: ChatTarget<'a>,
        message_id: MessageId,
        text: T,
    ) -> EditMessageText<'a> {
        EditMessageText {
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    pub fn disable_preview(self) -> EditMessageText<'a> {
        EditMessageText {
            disable_web_page_preview: Some(true),
            ..self
        }
    }

    pub fn parse_mode(self, mode: ParseMode) -> EditMessageText<'a> {
        EditMessageText {
            parse_mode: Some(mode),
            ..self
        }
    }

    pub fn reply_markup(self, markup: InlineKeyboardMarkup) -> Self {
        Self {
            reply_markup: Some(markup),
            ..self
        }
    }
}

/// Use this method to edit captions of messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`](types::Message) is
/// returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct EditMessageCaption<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatTarget<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> EditMessageCaption<'a> {
    pub fn new(chat_id: ChatTarget<'a>, message_id: MessageId) -> EditMessageCaption<'a> {
        EditMessageCaption {
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    pub fn caption(self, caption: String) -> EditMessageCaption<'a> {
        EditMessageCaption {
            caption: Some(caption),
            ..self
        }
    }

    pub fn parse_mode(self, mode: ParseMode) -> EditMessageCaption<'a> {
        EditMessageCaption {
            parse_mode: Some(mode),
            ..self
        }
    }

    pub fn reply_markup(self, markup: InlineKeyboardMarkup) -> Self {
        Self {
            reply_markup: Some(markup),
            ..self
        }
    }
}
/// Use this method to edit animation, audio, document, photo, or video messages. If a message is
/// a part of a message album, then it can be edited only to a photo or a video. Otherwise, message
/// type can be changed arbitrarily. When inline message is edited, new file can't be uploaded. Use
/// previously uploaded file via its file_id or specify a URL. On success, if the edited message was
/// sent by the bot, the edited [`Message`](types::Message) is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EditMessageMedia<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatTarget<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> EditMessageMedia<'a> {
    pub fn new(chat_id: ChatTarget<'a>, message_id: MessageId, media: InputMedia) -> EditMessageMedia<'a> {
        EditMessageMedia {
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
            media,
            reply_markup: None,
        }
    }
}

/// Use this method to edit only the reply markup of messages sent by the bot or via the bot (for
/// inline bots). On success, if edited message is sent by the bot, the edited [`Message`](types::Message)
/// is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct EditMessageReplyMarkup<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatTarget<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeleteMessage<'a> {
    pub chat_id: ChatTarget<'a>,
    pub message_id: MessageId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMe;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteWebhook;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWebhookInfo;

/// Telegram methods.
pub trait Method: Serialize {
    /// Method name in the Telegram Bot API url.
    const NAME: &'static str;
    /// Method return type.
    type Item: DeserializeOwned + fmt::Debug + 'static;

    /// Get method url.
    fn url(token: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", token, Self::NAME)
    }
}

#[rustfmt::skip]
impl_method_table!(
//  [               MethodType,      method_url_segment,          ApiReturnType],
    [                    GetMe,                 "getMe",            types::User],
    [            DeleteWebhook,         "deleteWebhook",                   bool],
    [           GetWebhookInfo,        "getWebhookInfo",     types::WebhookInfo],
    [           GetUpdates<'_>,            "getUpdates",     Vec<types::Update>],
    [           SetWebhook<'_>,            "setWebhook",                   bool],
    [          SendMessage<'_>,           "sendMessage",         types::Message],
    [       ForwardMessage<'_>,        "forwardMessage",         types::Message],
    [      EditMessageText<'_>,       "editMessageText",         types::Message],
    [        DeleteMessage<'_>,         "deleteMessage",                   bool],
    [   EditMessageCaption<'_>,    "editMessageCaption",                   bool],
    [          SendSticker<'_>,           "sendSticker",         types::Message],
    [            SendPhoto<'_>,             "sendPhoto",         types::Message],
    [         SendDocument<'_>,          "sendDocument",         types::Message],
    [              GetChat<'_>,               "getChat",            types::Chat],
    [GetChatAdministrators<'_>, "getChatAdministrators", Vec<types::ChatMember>],
    [  GetChatMembersCount<'_>,   "getChatMembersCount",                    i32],
    [        GetChatMember<'_>,         "getChatMember",      types::ChatMember]
);

// https://core.telegram.org/bots/api#making-requests
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TelegramResult<T> {
    pub ok: bool,
    pub description: Option<String>,
    pub error_code: Option<i32>,
    pub result: Option<T>,
    pub parameters: Option<types::ResponseParameters>,
}

impl<T> TelegramResult<T> {
    /// Convert the `TelegramResult` into `std` `Result`.
    pub fn into_result(self) -> Result<T, ApiError> {
        if self.ok {
            let api_error = ApiError {
                error_code: 0,
                description:
                    "In the response from telegram `ok: true`, but not found `result` field."
                        .to_string(),
                parameters: None,
            };
            self.result.ok_or(api_error)
        } else {
            let description = {
                if self.error_code.is_none() {
                    "In the response from telegram `ok: false`, but not found `err_code` field."
                        .to_string()
                } else {
                    self.description.unwrap_or_default()
                }
            };
            Err(ApiError {
                error_code: self.error_code.unwrap_or(0),
                description,
                parameters: self.parameters,
            })
        }
    }
}

impl<T> Into<Result<T, ApiError>> for TelegramResult<T> {
    fn into(self) -> Result<T, ApiError> {
        self.into_result()
    }
}

pub type UpdateList = TelegramResult<Vec<types::Update>>;

/// Types of updates.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum UpdateTypes {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message,
    /// New version of a message that is known to the bot and was edited
    EditedMessage,
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost,
    /// New version of a channel post that is known to the bot and was edited
    EditedChannelPost,
    /// New incoming inline query
    InlineQuery,
    /// The result of an inline query that was chosen by a user and sent to their chat partner.
    ChosenInlineResult,
    /// New incoming callback query
    CallbackQuery,
    /// New incoming shipping query. Only for invoices with flexible price
    ShippingQuery,
    /// New incoming pre-checkout query. Contains full information about checkout
    PreCheckoutQuery,
}
