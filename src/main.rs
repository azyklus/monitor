use std::{
   error::Error,
   io,
   thread,
   time::Duration,
};

/// A generic error type that *should* be able to be used with
/// most custom error implementations.
pub type GenericError = Box<dyn Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), GenericError>
{
   let _d = || async move {
      if let Err(why) = discord::setup().await {
         return Err(why);
      } else {
         return Ok(());
      }
   };

   thread::spawn(_d);

   return Ok(());
}


// CRATE MODULES ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains bot commands called on Discord or through Matrix.
///
/// Commands are prefixed with `mntr` and can be enumerated via
/// the `mntr help` command.
pub mod commands;

/// Contains the main logic relating to the Discord bot.
pub mod discord;


// CRATE DEPENDENCIES ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_use] extern crate anyhow;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serenity;
#[macro_use] extern crate tokio;
extern crate toml;
