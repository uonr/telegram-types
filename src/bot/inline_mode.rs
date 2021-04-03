//! The following methods and objects allow your bot to work in inline mode.
//!
//! Please see our [Introduction to Inline bots](https://core.telegram.org/bots/inline)
//! for more details.
//!
//! To enable this option, send the `/setinline` command to @BotFather and provide
//! the placeholder text that the user will see in the input field after typing
//! your bot’s name.

use super::types::{InlineKeyboardMarkup, Location, ParseMode, User};
use std::borrow::Cow;

/// Unique identifier for the answered query
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InlineQueryId(pub String);

/// Unique identifier for the result
///
/// 1-64 bytes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResultId(pub String);

/// An incoming inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: InlineQueryId,
    /// Sender
    pub from: Box<User>,
    /// Sender location, only for bots that request user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
    /// Text of the query (up to 512 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
}

/// Use this method to send answers to an inline query.
///
/// No more than 50 results per query are allowed.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerInlineQuery<'a> {
    /// Unique identifier for the answered query
    pub inline_query_id: InlineQueryId,
    /// A array of results for the inline query
    pub results: Cow<'a, [InlineQueryResult<'a>]>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached
    /// on the server. Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
    /// Pass True, if results may be cached on the server side only for the user that sent
    /// the query. By default, results may be returned to any user who sends the same query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to
    /// receive more results. Pass an empty string if there are no more results or if you don‘t
    /// support pagination. Offset length can’t exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<Cow<'a, str>>,
    /// If passed, clients will display a button with specified text that switches the user to
    /// a private chat with the bot and sends the bot a start message with the parameter
    /// switch_pm_parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<Cow<'a, str>>,
    /// Deep-linking parameter for the /start message sent to the bot when user presses the
    /// switch button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<Cow<'a, str>>,
}

impl_method!(AnswerInlineQuery<'_>, "answerInlineQuery", bool);

/// One result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InlineQueryResult<'a> {
    Article(InlineQueryResultArticle<'a>),
    // TODO: implement these placeholders
    #[doc(hidden)]
    Photo(()), // InlineQueryResultPhoto
    #[doc(hidden)]
    Gif(()), // InlineQueryResultGif
    #[doc(hidden)]
    Mpeg2Gif(()), // InlineQueryResultMpeg4Gif
    #[doc(hidden)]
    Video(()), // InlineQueryResultVideo
    #[doc(hidden)]
    Audio(()), // InlineQueryResultAudio
    #[doc(hidden)]
    Voice(()), // InlineQueryResultVoice
    #[doc(hidden)]
    Document(()), // InlineQueryResultDocument
    #[doc(hidden)]
    Location(()), // InlineQueryResultLocation
    #[doc(hidden)]
    Venue(()), // InlineQueryResultVenue
    #[doc(hidden)]
    Contact(()), // InlineQueryResultContact
    #[doc(hidden)]
    Game(()), // InlineQueryResultGame
}

/// A link to an article or web page.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQueryResultArticle<'a> {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: ResultId,
    /// Title of the result
    pub title: Cow<'a, str>,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent<'a>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// URL of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// Pass True, if you don't want the URL to be shown in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<bool>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<Cow<'a, str>>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i32>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i32>,
}

/// The content of a message to be sent as a result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum InputMessageContent<'a> {
    Text(InputTextMessageContent<'a>),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent<'a>),
    Contact(InputContactMessageContent<'a>),
}

/// The content of a text message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputTextMessageContent<'a> {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: Cow<'a, str>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}

/// The content of a location message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f32,
    /// Longitude of the location in degrees
    pub longitude: f32,
    /// Period in seconds for which the location can be updated, should be
    /// between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i32>,
}

/// The content of a venue message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputVenueMessageContent<'a> {
    /// Latitude of the venue in degrees
    pub latitude: f32,
    /// Longitude of the venue in degrees
    pub longitude: f32,
    /// Name of the venue
    pub title: Cow<'a, str>,
    /// Address of the venue
    pub address: Cow<'a, str>,
    /// Foursquare identifier of the venue, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<Cow<'a, str>>,
    /// Foursquare type of the venue, if known. (For example,
    /// “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<Cow<'a, str>>,
}

/// The content of a contact message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputContactMessageContent<'a> {
    /// Contact's phone number
    pub phone_number: Cow<'a, str>,
    /// Contact's first name
    pub first_name: Cow<'a, str>,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Cow<'a, str>>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<Cow<'a, str>>,
}

/// A result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: ResultId,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Identifier of the sent inline message. Available only if there is an
    /// [inline keyboard](InlineKeyboardMarkup) attached to the message. Will
    /// be also received in callback queries and can be used to edit the
    /// message.
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}
