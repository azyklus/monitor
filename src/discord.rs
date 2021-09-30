pub use self::app::Bot;

use crate::errors::GenericError;

use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;

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
   // Setup logging.
   if let Err(e) = setup_logging() {
      // Logging setup failed;
      // return an error.
      return Err(e.into());
   }

   // Setup the Discord bot itself.
   let mut bot: Bot = Bot::new().await.unwrap();
   if let Err(e) = bot.run().await {
      eprintln!("An error occurred while running the Discord bot!");

      // an error occurred while attempting to run the bot.
      return Err(e.into());
   }

   // all is okay!
   return Ok(bot);
}

/// Set up logging functionality for the Monitor application.
fn setup_logging() -> Result<(), fern::InitError>
{
   Dispatch::new()
      .format(|out, message, record| {
         out.finish(format_args!(
            "{}[{}][{}] {}",
            Local::now().format("[%Y-%m-%d] [%HH:%Mm:%Ss]"),
            record.target(),
            record.level(),
            message,
         ))
      })
      .level(LevelFilter::Debug)
      .chain(std::io::stdout())
      .chain(fern::log_file("discord.log")?)
      .apply()?;

   return Ok(());
}

/// Contains the main bot loop.
pub mod app;

/// Discord API configs.
pub mod config;
