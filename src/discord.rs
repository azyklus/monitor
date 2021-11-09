pub use self::app::Bot;

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
pub async fn setup(config: &DiscordConfig) -> Result<Bot, GenericError>
{
   // Setup the Discord bot itself.
   let mut bot: Bot = Bot::new(config).await.unwrap();
   if let Err(e) = bot.run().await {
      eprintln!("An error occurred while running the Discord bot!");

      // an error occurred while attempting to run the bot.
      return Err(e.into());
   }

   // all is okay!
   return Ok(bot);
}


use crate::errors::GenericError;

/// The main bot functionality.
pub mod app;

/// Discord API configs.
pub mod config;
use config::DiscordConfig;
