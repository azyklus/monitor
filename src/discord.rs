use std::io;
use super::GenericError;

pub use self::app::Bot;

pub async fn setup() -> Result<Bot, GenericError>
{
   let mut bot: Bot = Bot::new().await.unwrap();
   if let Err(e) = bot.run().await {
      return Err(e.into());
   }

   return Ok(bot);
}


/// Contains the main bot loop.
pub mod app;

/// Discord API configs.
pub mod config;

