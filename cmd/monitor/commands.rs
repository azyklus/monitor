// MODULES //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Chat management commands and functionality.
pub mod chat;
pub use chat::CHAT_GROUP;

/// Chat game commands.
pub mod games;
pub use games::GAMES_GROUP;

/// General-use commands.
pub mod general;
pub use general::MY_HELP;
pub use general::GENERAL_GROUP;

/// Owner-specific commands.
pub mod owner;
pub use owner::OWNER_GROUP;

/// XKCD comic displays.
pub mod xkcd;
pub use xkcd::XKCD_GROUP;
