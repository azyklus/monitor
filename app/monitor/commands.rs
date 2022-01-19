// MODULES ////////////////////////////////////////////////////////////////////////////////////////

/// Chat management commands and functionality.
pub mod chat;

/// Chat game commands.
pub mod games;

/// Commands that interface with the GIPHY API.
pub mod giphy;

/// General-use commands.
pub mod general;

/// Owner-specific commands.
pub mod owner;

/// XKCD comic displays.
pub mod xkcd;


// EXPORTS ////////////////////////////////////////////////////////////////////////////////////////

pub use self::{
   chat::CHAT_GROUP,
   games::GAMES_GROUP,
   general::{
      MY_HELP,
      GENERAL_GROUP,
   },
   giphy::GIPHY_GROUP,
   owner::OWNER_GROUP,
   xkcd::XKCD_GROUP,
};
