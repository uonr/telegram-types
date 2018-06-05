type Date = i64;
fn falsum() -> bool { false }


/// An incoming update.
///
/// At most one of the optional parameters can be present in any given update.
#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Update {
    /// The update‘s unique identifier.
    ///
    /// Update identifiers start from a certain positive number and increase sequentially.
    /// This ID becomes especially handy if you’re using Webhooks,
    /// since it allows you to ignore repeated updates or to restore the correct update sequence,
    /// should they get out of order. If there are no new updates for at least a week,
    /// then identifier of the next update will be chosen randomly instead of sequentially.
    update_id: i32,
    /// New incoming message of any kind — text, photo, sticker, etc.
    message: Option<Box<Message>>,
    /// New version of a message that is known to the bot and was edited
    edited_message: Option<Box<Message>>,
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    channel_post: Option<Box<Message>>,
    /// New version of a channel post that is known to the bot and was edited
    edited_channel_post: Option<Box<Message>>,
    // TODO: inline_query, chosen_inline_result, callback_query, shipping_query, pre_checkout_query
}


/// A Telegram user or bot.
#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: i32,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User‘s or bot’s first name
    pub first_name: String,
    /// User‘s or bot’s last name
    pub last_name: Option<String>,
    /// User‘s or bot’s username
    pub username:  Option<String>,
    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    pub language_code: Option<String>,
}


/// Type of chat
#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ChatKind {
    Private,
    Group,
    Supergroup,
    Channel,
}


#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,
    /// Type of chat
    #[serde(rename="type")]
    pub kind: ChatKind,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// True if a group has ‘All Members Are Admins’ enabled.
    all_members_are_administrators: Option<bool>,
    // TODO: photo
    /// Description, for supergroups and channel chats. Returned only in `getChat`.
    pub description: Option<String>,
    /// Chat invite link, for supergroups and channel chats. Returned only in getChat.
    pub invite_link: Option<String>,
    /// Pinned message, for supergroups and channel chats. Returned only in getChat.
    pub pinned_message: Option<Box<Message>>,
    /// For supergroups, name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
}


#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i32,
    /// Sender, empty for messages sent to channels
    pub from: Option<Box<User>>,
    /// Date the message was sent in Unix time
    pub date: Date,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// For forwarded messages, sender of the original message
    pub forward_from: Option<Box<User>>,
    /// For messages forwarded from channels, information about the original channel
    pub forward_from_chat: Option<Box<Chat>>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<i32>,
    /// For messages forwarded from channels, signature of the post author if present
    pub forward_signature: Option<String>,
    /// For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<Date>,
    /// For replies, the original message.
    /// Note that the Message object in this field will not contain
    /// further `reply_to_message` fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<Date>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    pub text: Option<String>,
    // TODO: audio, game, video, voice, video_note
    /// Message is a sticker, information about the sticker
    pub sticker: Option<Box<Sticker>>,
    /// Message is a general file, information about the file
    pub document: Option<Box<Document>>,
    pub photo: Vec<PhotoSize>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc.
    /// that appear in the text
    pub entities: Vec<MessageEntity>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc.
    /// that appear in the caption
    pub caption_entities: Vec<MessageEntity>,
    /// Caption for the audio, document, photo, video or voice, 0-200 characters
    pub caption: Option<String>,
    // TODO: contact, location, venue
    /// New members that were added to the group or supergroup and information about them
    /// (the bot itself may be one of these members)
    pub new_chat_members: Vec<User>,
    /// A member was removed from the group, information about them
    /// (this member may be the bot itself)
    pub left_chat_member: Option<Box<User>>,
    /// A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value
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
    pub migrate_to_chat_id: Option<i64>,
    /// The supergroup has been migrated from a group with the specified identifier.
    pub migrate_from_chat_id: Option<i64>,
    /// Specified message was pinned. Note that the Message object in this field
    /// will not contain further reply_to_message fields even if it is itself a reply.
    pub pinned_message: Option<Box<Message>>,
    // TODO: invoice, successful_payment
    /// The domain name of the website on which the user has logged in.
    pub connected_website: Option<String>,
}

/// One special entity in a text message.
/// For example, hashtags, usernames, URLs, etc.
#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageEntity {
    /// Type of the entity.
    pub kind: MessageEntityKind,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i32,
    /// Length of the entity in UTF-16 code units
    pub length: i32,
    /// For “text_link” only, url that will be opened after user taps on the text
    pub url: String,
    /// For “text_mention” only, the mentioned user
    pub user: Option<Box<User>>,
}

/// Type of the `MessageEntity`.
#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum  MessageEntityKind {
    /// `@username`
    Mention,
    Hashtag,
    BotCommand,
    Url,
    Email,
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
}

/// A general file (as opposed to photos, voice messages and audio files).
#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Document {
    /// Unique file identifier
    pub file_id: i32,
    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

/// One size of a photo or a file / sticker thumbnail.
#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhotoSize {
    /// Unique identifier for this file
    pub file_id: String,
    pub width: i32,
    pub height: i32,
    pub file_size: Option<i32>,
}

/// A user's profile pictures.
#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i32,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<PhotoSize>,
}


/// A [custom keyboard](https://core.telegram.org/bots#keyboards)
/// with reply options (see [Introduction to bots](https://core.telegram.org/bots#keyboards)
/// for details and examples).
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
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
    /// users that are @mentioned in the text of the Message object; 2)
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
#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Serialize, Deserialize,  Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sticker {
    pub file_id: String,
    pub width: i32,
    pub height: i32,
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    // TODO: mask_position
    /// File size
    file_size: i32,
}