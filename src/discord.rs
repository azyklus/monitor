pub use self::app::DiscordBot;

/// Sets up the Discord bot.
///
///
/// # Examples
///
/// ```should_fail
/// use crate::discord;
///
/// fn main()
/// {
/// ...
///    let bot = discord::setup(config).await.unwrap();
/// ...
/// }
/// ```
pub async fn setup(config: &DiscordConfig, fw: StandardFramework) -> Result<DiscordBot, GenericError>
{
   // Setup the Discord bot itself.
   let mut bot: DiscordBot = DiscordBot::new(config, fw).await.unwrap();
   if let Err(e) = bot.run().await {
      eprintln!("An error occurred while running the Discord bot!");

      // an error occurred while attempting to run the bot.
      return Err(e.into());
   }

   // all is okay!
   return Ok(bot);
}


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::errors::GenericError;

use serenity::framework::StandardFramework;


// MODULES //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The main bot functionality.
pub mod app;

/// Discord API configs.
pub mod config;
use config::DiscordConfig;

#[doc(hidden)]
pub mod hooks;
