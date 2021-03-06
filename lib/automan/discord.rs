// MODULES //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The main bot functionality.
pub mod app;

/// Discord API configs.
pub mod config;

#[doc(hidden)]
pub mod hooks;

/// Interactivity features for the Discord-Serenity API.
pub mod interactivity;

/// Pagination for the Discord-Serenity API.
pub mod pagination;

// EXPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub use self::{
   app::DiscordBot,
   config::DiscordConfig,
};
