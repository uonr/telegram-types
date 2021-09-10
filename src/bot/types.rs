//! Telegram bot object types.
use super::games::CallbackGame;
use super::inline_mode::{ChosenInlineResult, InlineQuery};
use super::utils::falsum;
#[cfg(feature = "high")]
use chrono::naive::NaiveDateTime;

macro_rules! impl_id {
    ($Id: ident : $Ty: ty) => {
        #[derive(
            Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
        )]
        pub struct $Id(pub $Ty);

        impl ::std::ops::Add<$Ty> for $Id {
            type Output = $Id;
            #[inline]
            fn add(self, other: $Ty) -> $Id {
                $Id(self.0 + other)
            }
        }

        impl<'a> ::std::ops::Add<&'a $Ty> for $Id {
            type Output = $Id;
            #[inline]
            fn add(self, other: &$Ty) -> Self::Output {
                $Id(self.0 + other)
            }
        }

        impl ::std::ops::Sub<$Ty> for $Id {
            type Output = $Id;
            #[inline]
            fn sub(self, other: $Ty) -> $Id {
                $Id(self.0 - other)
            }
        }
        impl ::std::ops::AddAssign<$Ty> for $Id {
            fn add_assign(&mut self, rhs: $Ty) {
                self.0 += rhs
            }
        }
        impl ::std::ops::SubAssign<$Ty> for $Id {
            fn sub_assign(&mut self, rhs: $Ty) {
                self.0 -= rhs
            }
        }
    };
}

impl_id! {UserId : i64}

impl_id! {ChatId : i64}

impl_id! {MessageId : i64}

impl_id! {UpdateId : i64}

/// Unique identifier for a file
/// # Sending by file_id
/// * It is not possible to change the file type when resending by **file_id**. I.e. a [video](Video) can't be sent as a photo, a [photo](Photo) can't be sent as a document, etc.
/// * It is not possible to resend thumbnails.
/// * Resending a photo by **file_id** will send all of its [sizes](PhotoSize).
/// * **file_id** is unique for each individual bot and can't be transferred from one bot to another.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub String);

/// This object represents a unique message identifier.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageIdResult {
    pub message_id: MessageId,
}

/// The UNIX timestamp
#[cfg(not(feature = "high"))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(pub u64);

/// The Datetime.
#[cfg(feature = "high")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(#[serde(with = "timestamp_format")] pub NaiveDateTime);

#[cfg(feature = "high")]
mod timestamp_format {
    use chrono::naive::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i64::deserialize(deserializer)?;
        Ok(NaiveDateTime::from_timestamp(s, 0))
    }
}

/// An incoming update.
///
/// At most one of the optional parameters can be present in any given update.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Update {
    /// The update‘s unique identifier.
    pub update_id: UpdateId,
    #[serde(flatten)]
    // `Option` is a workaround for https://github.com/serde-rs/serde/issues/1626
    pub content: Option<UpdateContent>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateContent {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message(Message),
    /// New version of a message that is known to the bot and was edited
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost(Message),
    /// New version of a channel post that is known to the bot and was edited
    EditedChannelPost(Message),
    /// New incoming inline query
    InlineQuery(InlineQuery),
    /// The result of an [inline](https://core.telegram.org/bots/api#inline-mode) query that
    /// was chosen by a user and sent to their chat partner.
    ///
    /// Please see our documentation on the
    /// [feedback collecting](https://core.telegram.org/bots/inline#collecting-feedback) for
    /// details on how to enable these updates for your bot.
    ChosenInlineResult(ChosenInlineResult),
    /// New incoming callback query
    CallbackQuery(CallbackQuery),
    // TODO: implement these placeholders
    #[doc(hidden)]
    ShippingQuery(ShippingQuery),
    #[doc(hidden)]
    PreCheckoutQuery(PreCheckoutQuery),
    #[doc(hidden)]
    Poll(Poll),
    #[doc(hidden)]
    PollAnswer(PollAnswer),
    #[doc(hidden)]
    MyChatMember(ChatMemberUpdated),
    #[doc(hidden)]
    ChatMember(ChatMemberUpdated),
    /// Unknown update type
    Unknown,
}
impl Default for UpdateContent {
    fn default() -> Self {
        UpdateContent::Unknown {}
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShippingQuery {}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PreCheckoutQuery {}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Poll {}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PollAnswer {}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatMemberUpdated {}

/// Contains information about the current status of a webhook.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i32,
    /// Currently used webhook IP address
    pub ip_address: Option<String>,
    /// Unix time for the most recent error that happened when trying to deliver an update via
    /// webhook
    pub last_error_date: Option<Time>,
    /// Error message in human-readable format for the most recent error that happened when trying
    /// to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    pub max_connections: Option<i32>,
    /// A list of update types the bot is subscribed to. Defaults to all update types
    pub allowed_updates: Option<Vec<String>>,
}

/// A Telegram user or bot.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: UserId,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User‘s or bot’s first name
    pub first_name: String,
    /// User‘s or bot’s last name
    pub last_name: Option<String>,
    /// User‘s or bot’s username
    pub username: Option<String>,
    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    pub language_code: Option<String>,
}

/// Type of chat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Private {
        username: Option<String>,
        /// First name of the other party in a private chat
        first_name: String,
        /// Last name of the other party in a private chat
        last_name: Option<String>,
    },
    Group {
        title: String,
        username: Option<String>,
        /// True if a group has ‘All Members Are Admins’ enabled.
        #[serde(default = "falsum")]
        all_members_are_administrators: bool,
    },
    Supergroup {
        title: String,
        username: Option<String>,
        /// True if a group has ‘All Members Are Admins’ enabled.
        #[serde(default = "falsum")]
        all_members_are_administrators: bool,
        /// Pinned message. Returned only in `getChat`.
        pinned_message: Option<Box<Message>>,
        /// Name of group sticker set. Returned only in `getChat.`
        sticker_set_name: Option<String>,
        /// True, if the bot can change the group sticker set. Returned only in `getChat`.
        can_set_sticker_set: Option<bool>,
        /// Chat invite link/ Returned only in `getChat`.
        invite_link: Option<String>,
        /// Description. Returned only in `getChat`.
        description: Option<String>,
    },
    Channel {
        title: String,
        username: Option<String>,
        /// Pinned message. Returned only in `getChat`.
        pinned_message: Option<Box<Message>>,
        /// Chat invite link. Returned only in `getChat`.
        invite_link: Option<String>,
        /// Description. Returned only in `getChat`.
        description: Option<String>,
    },
    #[serde(other)]
    /// Unknown upstream data type.
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: ChatId,
    /// Chat photo. Returned only in `getChat`.
    pub photo: Option<ChatPhoto>,
    /// Type of chat
    #[serde(flatten)]
    #[serde(rename = "type")]
    pub kind: ChatType,
}

// TODO: game, invoice, successful_payment
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: MessageId,
    /// Sender, empty for messages sent to channels
    pub from: Option<Box<User>>,
    /// Sender of the message, sent on behalf of a chat.
    /// The channel itself for channel messages.
    /// The supergroup itself for messages from anonymous group administrators.
    /// The linked channel for messages automatically forwarded to the discussion group
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: Time,
    /// Conversation the message belongs to
    pub chat: Box<Chat>,
    /// For forwarded messages, sender of the original message
    pub forward_from: Option<Box<User>>,
    /// For messages forwarded from channels, information about the original channel
    pub forward_from_chat: Option<Box<Chat>>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<MessageId>,
    /// For messages forwarded from channels, signature of the post author if present
    pub forward_signature: Option<String>,
    /// Sender's name for messages forwarded from users who disallow adding a link to their account
    /// in forwarded messages
    pub forward_sender_name: Option<String>,
    /// For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<Time>,
    /// For replies, the original message.
    /// Note that the Message object in this field will not contain
    /// further `reply_to_message` fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<Time>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    pub text: Option<String>,
    /// Message is a sticker, information about the sticker
    pub sticker: Option<Box<Sticker>>,
    /// Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file
    pub document: Option<Box<Document>>,
    #[serde(default)]
    pub photo: Vec<PhotoSize>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc.
    /// that appear in the text
    #[serde(default)]
    pub entities: Vec<MessageEntity>,
    /// Message is a voice message, information about the file
    pub voice: Option<Box<Voice>>,
    /// Message is a video, information about the video
    pub video: Option<Video>,
    /// Message is a video note, information about the video message
    pub video_note: Option<Box<VideoNote>>,
    /// Message is an animation, information about the animation.
    ///
    /// For backward compatibility, when this field is set, the document field will also be set
    pub animation: Option<Box<Animation>>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc.
    /// that appear in the caption
    #[serde(default)]
    pub caption_entities: Vec<MessageEntity>,
    /// Caption for the audio, document, photo, video or voice, 0-200 characters
    pub caption: Option<String>,
    /// Message is a shared contact, information about the contact
    pub contact: Option<Box<Contact>>,
    /// Message is a shared location, information about the location
    pub location: Option<Box<Location>>,
    /// Message is a venue, information about the venue
    pub venue: Option<Box<Venue>>,
    /// New members that were added to the group or supergroup and information about them
    /// (the bot itself may be one of these members)
    #[serde(default)]
    pub new_chat_members: Vec<User>,
    /// A member was removed from the group, information about them
    /// (this member may be the bot itself)
    pub left_chat_member: Option<Box<User>>,
    /// A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value
    #[serde(default)]
    pub new_chat_photo: Vec<PhotoSize>,
    /// Service message: the chat photo was deleted
    #[serde(default = "falsum")]
    pub delete_chat_photo: bool,
    /// Service message: the group has been created
    #[serde(default = "falsum")]
    pub group_chat_created: bool,
    /// Service message: the supergroup has been created.
    /// This field can‘t be received in a message coming through updates, because bot can’t
    /// be a member of a supergroup when it is created. It can only be found in reply_to_message
    /// if someone replies to a very first message in a directly created supergroup.
    #[serde(default = "falsum")]
    pub supergroup_chat_created: bool,
    /// Service message: the channel has been created.
    ///
    /// This field can‘t be received in a message coming through updates, because bot can’t be
    /// a member of a channel when it is created. It can only be found in reply_to_message
    /// if someone replies to a very first message in a channel.
    #[serde(default = "falsum")]
    pub channel_chat_created: bool,
    /// The group has been migrated to a supergroup with the specified identifier.
    pub migrate_to_chat_id: Option<ChatId>,
    /// The supergroup has been migrated from a group with the specified identifier.
    pub migrate_from_chat_id: Option<ChatId>,
    /// Specified message was pinned. Note that the Message object in this field
    /// will not contain further reply_to_message fields even if it is itself a reply.
    pub pinned_message: Option<Box<Message>>,
    /// The domain name of the website on which the user has logged in.
    pub connected_website: Option<String>,
    /// Inline keyboard attached to the message.
    ///
    /// `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// One special entity in a text message.
/// For example, hashtags, usernames, URLs, etc.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MessageEntity {
    /// Type of the entity.
    #[serde(rename = "type")]
    pub kind: MessageEntityKind,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i32,
    /// Length of the entity in UTF-16 code units
    pub length: i32,
    /// For “text_link” only, url that will be opened after user taps on the text
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user
    pub user: Option<Box<User>>,
}

/// Type of the `MessageEntity`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityKind {
    /// `@username`
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    /// bold text
    Bold,
    /// italic text
    Italic,
    /// monowidth string
    Code,
    /// monowidth block
    Pre,
    /// for clickable text URLs
    TextLink,
    /// for users without usernames
    TextMention,
    #[serde(other)]
    /// Unknown upstream data type.
    Unknown,
}

/// A general file (as opposed to [photos](PhotoSize), [voice messages](Voice) and
/// [audio files](Audio)).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Document {
    /// Unique file identifier
    pub file_id: FileId,
    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

/// A video file.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Video {
    /// Unique identifier for this file
    pub file_id: FileId,
    pub width: i32,
    pub height: i32,
    /// Duration of the video in seconds as defined by sender
    pub duration: i32,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<i32>,
}

/// An animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Animation {
    /// Unique identifier for this file
    pub file_id: FileId,
    pub width: i32,
    pub height: i32,
    /// Duration of the video in seconds as defined by sender
    pub duration: i32,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<i32>,
}

/// An audio file to be treated as music by the Telegram clients.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Audio {
    /// Unique identifier for this file
    pub file_id: FileId,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i32,
    /// Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
    /// Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}

/// A voice note.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Voice {
    /// Unique identifier for this file
    pub file_id: FileId,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i32,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

/// A video message (available in Telegram apps as of v.4.0).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VideoNote {
    /// Unique identifier for this file
    pub file_id: FileId,
    /// Video width and height as defined by sender
    pub length: i32,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i32>,
}

/// A phone contact.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<UserId>,
    /// Additional data about the contact in the form of a vCard
    pub vcard: Option<String>,
}

/// A file ready to be downloaded.
/// The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`.
/// It is guaranteed that the link will be valid for at least 1 hour. When the link expires,
/// a new one can be requested by calling `getFile`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
pub struct File {
    /// Unique identifier for this file
    pub file_id: FileId,
    /// File size, if known
    pub file_size: Option<i32>,
    /// Optional. File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
    pub file_path: Option<String>,
}

/// A point on the map.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f32,
    /// Latitude as defined by sender
    pub latitude: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Venue {
    /// Venue location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue. (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
}

/// One size of a photo or a [file](Document) / [sticker](Sticker) thumbnail.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhotoSize {
    /// Unique identifier for this file
    pub file_id: FileId,
    pub width: i32,
    pub height: i32,
    pub file_size: Option<i32>,
}

/// A user's profile pictures.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i32,
    /// Requested profile pictures (in up to 4 sizes each)
    #[serde(default)]
    pub photos: Vec<PhotoSize>,
}

/// A [custom keyboard](https://core.telegram.org/bots#keyboards)
/// with reply options (see [Introduction to bots](https://core.telegram.org/bots#keyboards)
/// for details and examples).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [`KeyboardButton`](KeyboardButton) objects
    #[serde(default)]
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of buttons).
    /// Defaults to false, in which case the custom keyboard is always of the
    /// same height as the app's standard keyboard.
    pub resize_keyboard: Option<bool>,
    /// Requests clients to hide the keyboard as soon as it's been used.
    /// The keyboard will still be available, but clients will automatically display the usual
    /// letter-keyboard in the chat – the user can press a special button in the input field
    /// to see the custom keyboard again. Defaults to `false`.
    pub one_time_keyboard: Option<bool>,
    /// Use this parameter if you want to show the keyboard to specific users only. Targets: 1)
    /// users that are @mentioned in the text of the [`Message`] object; 2)
    /// if the bot's message is a reply (has reply_to_message_id),
    /// sender of the original message.
    ///
    /// Example: A user requests to change the bot‘s language,
    /// bot replies to the request with a keyboard to select the new language.
    /// Other users in the group don’t see the keyboard.
    pub selective: Option<bool>,
}

/// One button of the reply keyboard.
/// For simple text buttons *String* can be used instead of this object to specify
/// text of the button. Optional fields are mutually exclusive.
///
/// ## Note
/// Note: request_contact and request_location options will only work in
/// Telegram versions released after 9 April, 2016. Older clients will ignore them.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used,
    /// it will be sent as a message when the button is pressed
    pub text: String,
    /// If True, the user's phone number will be sent as a contact when the button is pressed.
    /// Available in private chats only
    pub request_contact: Option<bool>,
    /// If True, the user's current location will be sent when the button is pressed.
    /// Available in private chats only
    pub request_location: Option<bool>,
}

/// Upon receiving a message with this object, Telegram clients will remove the current
/// custom keyboard and display the default letter-keyboard.
///
/// By default, custom keyboards are displayed until a new keyboard is sent by a bot.
/// An exception is made for one-time keyboards that are hidden immediately after the user
/// presses a button (see [`ReplyKeyboardMarkup`]).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to summon this
    /// keyboard; if you want to hide the keyboard from sight but keep it accessible,
    /// use *one_time_keyboard* in [`ReplyKeyboardMarkup`])
    pub remove_keyboard: bool,
    /// *Optional*. Use this parameter if you want to remove the keyboard for specific users only.
    /// Targets:
    ///
    /// 1. users that are @mentioned in the text of the Message object;
    /// 2. if the bot's message is a reply (has reply_to_message_id),
    /// sender of the original message.
    ///
    /// *Example*: A user votes in a poll, bot returns confirmation message in reply to the
    /// vote and removes the keyboard for that user, while still showing the keyboard with poll
    /// options to users who haven't voted yet.
    pub selective: Option<bool>,
}

/// An inline keyboard that appears right next to the message it belongs to.
///
/// ## Note
/// This will only work in Telegram versions released after 9 April, 2016.
/// Older clients will display unsupported message.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [`InlineKeyboardButton`] objects
    #[serde(default)]
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// One button of an inline keyboard.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    #[serde(flatten)]
    pub pressed: InlineKeyboardButtonPressed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InlineKeyboardButtonPressed {
    /// HTTP url to be opened when button is pressed
    Url(String),
    /// Data to be sent in a [callback query](CallbackQuery) to the bot when button is pressed,
    /// 1-64 bytes
    CallbackData(String),
    /// If set, pressing the button will prompt the user to select one of their chats, open that
    /// chat and insert the bot‘s username and the specified inline query in the input field.
    /// Can be empty, in which case just the bot’s username will be inserted.
    ///
    /// ## Note
    /// This offers an easy way for users to start using your bot in
    /// [inline mode](https://core.telegram.org/bots/inline) when they are currently
    /// in a private chat with it. Especially useful when combined with
    /// *[switch_pm…](https://core.telegram.org/bots/api#answerinlinequery)* actions – in this
    /// case the user will be automatically returned to the chat they switched from, skipping
    /// the chat selection screen.
    SwitchInlineQuery(String),
    /// If set, pressing the button will insert the bot‘s username and the specified inline
    /// query in the current chat's input field. Can be empty, in which case only
    /// the bot’s username will be inserted.
    ///
    /// This offers a quick way for the user to open your bot in inline mode in the same chat –
    /// good for selecting something from multiple options.
    SwitchInlineQueryCurrentChat(String),
    /// Description of the game that will be launched when the user presses the button.
    ///
    /// # NOTE
    /// This type of button **must** always be the first button in the first row.
    Pay(bool),
    /// Description of the game that will be launched when the user presses the button.
    ///
    /// ## NOTE
    /// This type of button **must** always be the first button in the first row.
    CallbackGame(CallbackGame),
    /// An HTTP URL used to automatically authorize the user.
    LoginUrl(LoginUrl),
    #[serde(other)]
    /// Unknown upstream data type.
    Unknown,
}

/// This object represents an incoming callback query from a callback button in an inline keyboard.
/// If the button that originated the query was attached to a message sent by the bot, the field
/// message will be present. If the button was attached to a message sent via the bot (in inline
/// mode), the field inline_message_id will be present. Exactly one of the fields data
/// or game_short_name will be present.
///
/// ## Note
/// After the user presses a callback button, Telegram clients will display a progress bar until
/// you call `answerCallbackQuery`. It is, therefore, necessary to react by calling
/// `answerCallbackQuery` even if no notification to the user is needed (e.g., without
/// specifying any of the optional parameters).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: Box<User>,
    /// Message with the callback button that originated the query. Note that message content and
    /// message date will not be available if the message is too old
    pub message: Option<Box<Message>>,
    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the
    /// callback button was sent. Useful for high scores in games.
    pub chat_instance: String,
    /// Data associated with the callback button. Be aware that a bad client can send arbitrary data in this field.
    pub data: Option<String>,
    /// Short name of a Game to be returned, serves as the unique identifier for the game
    pub game_short_name: Option<String>,
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface
/// to the user (act as if the user has selected the bot‘s message and tapped ’Reply'). This can
/// be extremely useful if you want to create user-friendly step-by-step interfaces without having
/// to sacrifice [privacy mode](https://core.telegram.org/bots#privacy-mode).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot‘s message and
    /// tapped ’Reply'
    pub force_reply: bool,
    /// *Optional*. Use this parameter if you want to force reply from specific users only.
    /// Targets:
    ///
    /// 1. users that are @mentioned in the text of the [`Message`] object;
    /// 2. if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    pub selective: Option<bool>,
}

/// Contains information about why a request was unsuccessful.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ResponseParameters {
    /// *Optional*. The group has been migrated to a supergroup with the specified identifier.
    pub migrate_to_chat_id: Option<ChatId>,
    /// In case of exceeding flood control, the number of seconds left to wait before the request
    /// can be repeated
    pub retry_after: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatPhoto {
    /// Unique file identifier of small (160x160) chat photo.
    /// This file_id can be used only for photo download.
    pub small_file_id: FileId,
    /// Unique file identifier of big (640x640) chat photo.
    /// This file_id can be used only for photo download.
    pub big_file_id: FileId,
}

/// This object contains information about one member of a chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatMember {
    /// Information about the user
    pub user: Box<User>,
    /// The member's status in the chat.
    pub status: ChatMemberStatus,
    /// Restricted and kicked only. Date when restrictions will be lifted for this user, unix time
    pub until_date: Option<Time>,
    /// Administrators only. True, if the bot is allowed to edit administrator privileges of
    /// that user
    pub can_be_edited: Option<bool>,
    /// Administrators only. True, if the administrator can change the chat title, photo and
    /// other settings
    pub can_change_info: Option<bool>,
    /// Administrators only. True, if the administrator can post in the channel, channels only
    pub can_post_messages: Option<bool>,
    /// Administrators only. True, if the administrator can edit messages of other users and can
    /// pin messages, channels only
    pub can_edit_messages: Option<bool>,
    /// Administrators only. True, if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    /// Administrators only. True, if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// Administrators only. True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Option<bool>,
    /// Administrators only. True, if the administrator can pin messages, supergroups only
    pub can_pin_messages: Option<bool>,
    /// Administrators only. True, if the administrator can add new administrators with a subset
    /// of his own privileges or demote administrators that he has promoted, directly or
    /// indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: Option<bool>,
    /// Restricted only. True, if the user is a member of the chat at the moment of the request
    pub is_member: Option<bool>,
    /// Restricted only. True, if the user can send text messages, contacts, locations and venues
    pub can_send_messages: Option<bool>,
    /// Restricted only. True, if the user can send audios, documents, photos, videos, video notes
    /// and voice notes, implies can_send_messages
    pub can_send_media_messages: Option<bool>,
    /// Restricted only. True, if the user can send animations, games, stickers and use inline
    /// bots, implies can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    /// Restricted only. True, if user may add web page previews to his messages, implies
    /// can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
}

/// The member's status in the chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum ChatMemberStatus {
    Creator,
    Administrator,
    Member,
    Restricted,
    Left,
    Kicked,
    #[serde(other)]
    /// Unknown upstream data type.
    Unknown,
}

/// The contents of a file to be uploaded.
///
/// Must be posted using `multipart/form-data` in the usual way that
/// files are uploaded via the browser.
///
/// [More info on Sending Files](https://core.telegram.org/bots/api#sending-files)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputFile(pub String);

impl InputFile {
    /// using `multipart/form-data` under <file_attach_name> name.
    pub fn new<S: AsRef<str>>(file_attach_name: S) -> InputFile {
        let attach = format!("attach://{}", file_attach_name.as_ref());
        InputFile(attach)
    }
}

/// There are three ways to send files
///
/// 1. If the file is already stored somewhere on the Telegram servers, you don't need to reupload it: each file object has a **file_id** field, simply pass this **file_id** as a parameter instead of uploading. There are **no limits** for files sent this way.
/// 2. Provide Telegram with an HTTP URL for the file to be sent. Telegram will download and send the file. 5 MB max size for photos and 20 MB max for other types of content.
/// 3. Post the file using multipart/form-data in the usual way that files are uploaded via the browser. 10 MB max size for photos, 50 MB for other files.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum FileToSend {
    FileId(FileId),
    Url(String),
    InputFile(InputFile),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sticker {
    pub file_id: FileId,
    pub width: i32,
    pub height: i32,
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// File size
    pub file_size: i32,
}

/// A sticker set.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// *True*, if the sticker set contains masks
    pub contains_masks: bool,
    /// List of all set stickers
    #[serde(default)]
    pub stickers: Vec<Sticker>,
}

/// The position on faces where a mask should be placed by default.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”,
    /// “mouth”, or “chin”.
    pub point: String,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right.
    /// For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f32,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom.
    /// For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f32,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f32,
}

/// The content of a media message to be sent.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "video")]
    Video {
        /// File to send.
        ///
        /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet, or pass
        /// "attach://<file_attach_name>" to upload a new one using multipart/form-data
        /// under <file_attach_name> name.
        ///
        /// [More info on Sending Files](https://core.telegram.org/bots/api#sending-files)
        media: FileToSend,
        /// *Optional*. Caption of the photo to be sent, 0-200 characters
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// *Optional*. Send Markdown or HTML, if you want Telegram apps to show
        /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options)
        /// in the media caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i32>,
        /// Pass True, if the uploaded video is suitable for streaming
        #[serde(skip_serializing_if = "Option::is_none")]
        supports_streaming: Option<bool>,
    },
    #[serde(rename = "photo")]
    Photo {
        /// File to send.
        ///
        /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet, or pass
        /// "attach://<file_attach_name>" to upload a new one using multipart/form-data
        /// under <file_attach_name> name.
        ///
        /// [More info on Sending Files](https://core.telegram.org/bots/api#sending-files)
        media: FileToSend,
        /// *Optional*. Caption of the photo to be sent, 0-200 characters
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// *Optional*. Send Markdown or HTML, if you want Telegram apps to show
        /// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options)
        /// in the media caption.
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
    },
    #[serde(rename = "animation")]
    Animation {
        /// File to send.
        ///
        /// Pass a file_id to send a file that exists on the Telegram servers (recommended),
        /// pass an HTTP URL for Telegram to get a file from the Internet, or pass
        /// "attach://<file_attach_name>" to upload a new one using multipart/form-data
        /// under <file_attach_name> name.
        ///
        /// [More info on Sending Files](https://core.telegram.org/bots/api#sending-files)
        media: FileToSend,
        /// Thumbnail of the file sent.
        ///
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        ///
        /// A thumbnail‘s width and height should not exceed 90.
        ///
        /// Ignored if the file is not uploaded using multipart/form-data.
        ///
        /// Thumbnails can’t be reused and can be only uploaded as a new file,
        /// so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded
        /// using multipart/form-data under <file_attach_name>.
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFile>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i32>,
    },
    #[serde(rename = "audio")]
    Audio {
        media: FileToSend,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFile>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    #[serde(rename = "document")]
    Document {
        media: FileToSend,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFile>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
    },
    #[serde(other)]
    /// Unknown upstream data type.
    Unknown,
}

/// a parameter of the inline keyboard button used to automatically authorize a user.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LoginUrl {
    /// An HTTP URL to be opened with user authorization data added to the query string when
    /// the button is pressed.
    ///
    /// If the user refuses to provide authorization data, the original URL without information
    /// about the user will be opened. The data added is the same as described in
    /// [Receiving authorization data](https://core.telegram.org/widgets/login#receiving-authorization-data).
    ///
    /// ## Note
    ///
    /// You **must** always check the hash of the received data to verify the authentication and
    /// the integrity of the data as described in [Checking authorization](https://core.telegram.org/widgets/login#checking-authorization).
    pub url: String,
    /// New text of the button in forwarded messages.
    pub forward_text: Option<String>,
    /// Username of a bot, which will be used for user authorization.
    ///
    /// See Setting up a bot for more details.
    /// If not specified, the current bot's username will be assumed.
    /// The *url*'s domain must be the same as the domain linked with the bot.
    /// See [Linking your domain to the bot](https://core.telegram.org/widgets/login#linking-your-domain-to-the-bot)
    /// for more details.
    pub bot_username: Option<String>,
    /// Pass True to request the permission for your bot to send messages to the user.
    pub request_write_access: Option<bool>,
}

/// Send *Markdown or HTML*, if you want Telegram apps to show
/// [bold, italic, fixed-width text or inline URLs](https://core.telegram.org/bots/api#formatting-options)
/// in the media caption.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ParseMode {
    Markdown,
    MarkdownV2,
    HTML,
    #[serde(other)]
    /// Unknown upstream data type.
    Unknown,
}
