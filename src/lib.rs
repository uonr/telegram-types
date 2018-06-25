//! Types in the Telegram Bot API and their deserializers
//!
//! See also [Telegram Bot API](https://core.telegram.org/bots/api).
#[cfg(feature = "high")]
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod bot;


