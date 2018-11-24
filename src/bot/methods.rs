//! Request parameters types of Telegram bot methods.
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::borrow::Cow;
use super::types;
use super::types::{ChatId, ForceReply, InlineKeyboardMarkup, MessageId,
                   ParseMode, ReplyKeyboardMarkup, ReplyKeyboardRemove, UpdateId, UserId};


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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetWebhook<'a> {
    pub url: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Cow<'a, [UpdateTypes]>>,
}


impl<'a> SetWebhook<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(url: T) -> SetWebhook<'a> {
        SetWebhook {
            url: url.into(),
            max_connections: None,
            allowed_updates: None,
        }
    }

    pub fn max_connections(self, x: i32) -> SetWebhook<'a> {
        SetWebhook { max_connections: Some(x), ..self }
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
        SendMessage { parse_mode: Some(mode), ..self }
    }

    pub fn reply(self, message_id: MessageId) -> SendMessage<'a> {
        SendMessage {
            reply_to_message_id: Some(message_id),
            ..self
        }
    }

    pub fn reply_markup(self, markup: ReplyMarkup) -> Self {
        Self { reply_markup: Some(markup), ..self }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum File {
    Id(types::FileId),
    Url(String),
}


/// Use this method to send .webp stickers.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendSticker<'a> {
    pub chat_id: ChatTarget<'a>,
    pub sticker: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}


impl<'a> SendSticker<'a> {
    pub fn new(chat_id: ChatTarget<'a>, sticker: File) -> SendSticker<'a> {
        SendSticker {
            chat_id,
            sticker,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn reply(self, reply_to_message_id: MessageId) -> SendSticker<'a> {
        SendSticker { reply_to_message_id: Some(reply_to_message_id), ..self }
    }


    pub fn reply_markup(self, markup: ReplyMarkup) -> Self {
        Self { reply_markup: Some(markup), ..self }
    }
}


/// Use this method to send photos.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendPhoto<'a> {
    pub chat_id: ChatTarget<'a>,
    pub photo: File,
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
    pub fn new(chat_id: ChatTarget<'a>, photo: File) -> SendPhoto<'a> {
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
        SendPhoto { parse_mode: Some(mode), ..self }
    }


    pub fn reply(self, reply_to_message_id: MessageId) -> SendPhoto<'a> {
        SendPhoto { reply_to_message_id: Some(reply_to_message_id), ..self }
    }


    pub fn reply_markup(self, markup: ReplyMarkup) -> Self {
        Self { reply_markup: Some(markup), ..self }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendDocument<'a> {
    pub chat_id: ChatTarget<'a>,
    pub document: File,
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
    pub fn new(chat_id: ChatTarget<'a>, document: File) -> SendDocument<'a> {
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
        SendDocument { parse_mode: Some(mode), ..self }
    }

    pub fn reply(self, reply_to_message_id: MessageId) -> SendDocument<'a> {
        SendDocument { reply_to_message_id: Some(reply_to_message_id), ..self }
    }


    pub fn reply_markup(self, markup: ReplyMarkup) -> Self {
        Self { reply_markup: Some(markup), ..self }
    }
}


/// Use this method to forward messages of any kind.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForwardMessage<'a> {
    pub chat_id: ChatTarget<'a>,
    pub from_chat_id: ChatTarget<'a>,
    pub message_id: MessageId,
}

/// To get a list of profile pictures for a user. Returns a [`UserProfilePhotos`](types::UserProfilePhotos) object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChat<'a> {
    pub chat_id: ChatTarget<'a>,
}

/// Use this method to get the number of members in a chat. Returns `Int` on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMembersCount<'a> {
    pub chat_id: ChatTarget<'a>,
}

/// Use this method to get a list of administrators in a chat. On success, returns an Array
/// of `ChatMember` objects that contains information about all chat administrators except
/// other bots. If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatAdministrators<'a> {
    pub chat_id: ChatTarget<'a>,
}

/// Use this method to get information about a member of a chat. Returns a `ChatMember`
/// object on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMember<'a> {
    pub chat_id: ChatTarget<'a>,
    pub user_id: UserId,
}


/// Use this method to edit text and game messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`](types::Message) is
/// returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub fn new<T: Into<Cow<'a, str>>>(chat_id: ChatTarget<'a>, message_id: MessageId, text: T) -> EditMessageText<'a> {
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
        EditMessageText { disable_web_page_preview: Some(true), ..self }
    }

    pub fn parse_mode(self, mode: ParseMode) -> EditMessageText<'a> {
        EditMessageText { parse_mode: Some(mode), ..self }
    }

    pub fn reply_markup(self, markup: InlineKeyboardMarkup) -> Self {
        Self { reply_markup: Some(markup), ..self }
    }
}



/// Use this method to edit captions of messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`](types::Message) is
/// returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
        EditMessageCaption { caption: Some(caption), ..self }
    }

    pub fn parse_mode(self, mode: ParseMode) -> EditMessageCaption<'a> {
        EditMessageCaption { parse_mode: Some(mode), ..self }
    }

    pub fn reply_markup(self, markup: InlineKeyboardMarkup) -> Self {
        Self { reply_markup: Some(markup), ..self }
    }
}


/// Use this method to edit only the reply markup of messages sent by the bot or via the bot (for
/// inline bots). On success, if edited message is sent by the bot, the edited [`Message`](types::Message)
/// is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[macro_export]
macro_rules! impl_method {
    ($Type: ty, $lifetime: lifetime, $name: expr, $Item: ty) => {
        impl<$lifetime> $crate::bot::methods::Method for $Type {
            const NAME: &'static str = $name;
            type Item = $Item;
        }
    };
    ($Type: ty, $name: expr, $Item: ty) => {
        impl $crate::bot::methods::Method for $Type {
            const NAME: &'static str = $name;
            type Item = $Item;
        }
    };
}


//           Type                           Method                   Return
impl_method!(GetMe                        , "getMe"                , types::User           );
impl_method!(DeleteWebhook                , "deleteWebhook"        , bool                  );
impl_method!(GetWebhookInfo               , "getWebhookInfo"       , types::WebhookInfo    );
impl_method!(GetUpdates<'a>           , 'a, "getUpdates"           , Vec<types::Update>    );
impl_method!(SetWebhook<'a>           , 'a, "setWebhook"           , bool                  );
impl_method!(SendMessage<'a>          , 'a, "sendMessage"          , types::Message        );
impl_method!(ForwardMessage<'a>       , 'a, "forwardMessage"       , types::Message        );
impl_method!(EditMessageText<'a>      , 'a, "editMessageText"      , types::Message        );
impl_method!(DeleteMessage<'a>        , 'a, "deleteMessage"        , bool                  );
impl_method!(EditMessageCaption<'a>   , 'a, "editMessageCaption"   , bool                  );
impl_method!(SendSticker<'a>          , 'a, "sendSticker"          , types::Message        );
impl_method!(SendPhoto<'a>            , 'a, "sendPhoto"            , types::Message        );
impl_method!(SendDocument<'a>         , 'a, "sendDocument"         , types::Message        );
impl_method!(GetChat<'a>              , 'a, "getChat"              , types::Chat           );
impl_method!(GetChatAdministrators<'a>, 'a, "getChatAdministrators", Vec<types::ChatMember>);
impl_method!(GetChatMembersCount<'a>  , 'a, "getChatMembersCount"  , i32                   );
impl_method!(GetChatMember<'a>        , 'a, "getChatMember"        , types::ChatMember     );

// https://core.telegram.org/bots/api#making-requests
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TelegramResult<T>
{
    pub ok: bool,
    pub description: Option<String>,
    pub err_code: Option<i32>,
    pub result: Option<T>,
    pub parameters: Option<types::ResponseParameters>,
}


impl<T> Into<Result<T, ApiError>> for TelegramResult<T> {
    fn into(self) -> Result<T, ApiError> {
        if self.ok {
            let api_error = ApiError {
                error_code: 0,
                description: "In the response from telegram `ok: true`, but not found `result` field.".to_string(),
                parameters: None,
            };
            self.result.ok_or(api_error)
        } else {
            let description = {
                if self.err_code.is_none() {
                    "In the response from telegram `ok: false`, but not found `err_code` field.".to_string()
                } else {
                    self.description.unwrap_or_default()
                }
            };
            Err(
                ApiError {
                    error_code: self.err_code.unwrap_or(0),
                    description,
                    parameters: self.parameters
                }
            )
        }
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
