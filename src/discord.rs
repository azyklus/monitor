use std::io;
use super::GenericError;

pub use self::app::Bot;

pub async fn setup() -> Result<Bot, GenericError>
{
   Ok(Bot{})
}


/// Contains the main bot loop.
pub mod app;

/// Discord API configs.
pub mod config;

