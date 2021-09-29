use crate::errors::GenericError;

pub use self::app::Bot;

/// Sets up the Discord bot.
///
///
/// # Examples
///
/// ```
/// use crate::discord;
///
/// fn main()
/// {
/// ...
///    let bot = discord::setup().await.unwrap();
/// ...
/// }
/// ```
pub async fn setup() -> Result<Bot, GenericError>
{
   let mut bot: Bot = Bot::new().await.unwrap();
   if let Err(e) = bot.run().await {
      eprintln!("An error occurred while running the Discord bot!");

      return Err(e.into());
   }

   return Ok(bot);
}


/// Contains the main bot loop.
pub mod app;

/// Discord API configs.
pub mod config;
