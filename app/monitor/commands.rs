// MODULES ////////////////////////////////////////////////////////////////////////////////////////

/// Chat management commands and functionality.
pub mod chat;

/// Commands for managing configuration details of the app.
pub mod config;

/// Commands for general amusement.
pub mod fun;

/// Chat game commands.
pub mod games;

/// Commands that interface with the GIPHY API.
pub mod giphy;

/// General-use commands.
pub mod general;

/// Owner-specific commands.
pub mod owner;

/// Commands for interacting with Twitter.
pub mod twitter;

/// XKCD comic displays.
pub mod xkcd;


// EXPORTS ////////////////////////////////////////////////////////////////////////////////////////

pub use self::{
   chat::CHAT_GROUP,
   config::CONFIG_GROUP,
   fun::FUN_GROUP,
   games::GAMES_GROUP,
   general::{
      MY_HELP,
      GENERAL_GROUP,
   },
   giphy::GIPHY_GROUP,
   owner::OWNER_GROUP,
   twitter::TWITTER_GROUP,
   xkcd::XKCD_GROUP,
};
