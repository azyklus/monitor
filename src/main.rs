use std::{
   error::Error,
   io,
};

/// A generic error type that *should* be able to be used with
/// most custom error implementations.
pub type GenericError = Box<dyn Error + Send + Sync>;

#[tokio::main]
async fn main() -> io::Result<()>
{

}

/// Contains the main logic relating to the Discord bot.
pub mod discord;

#[macro_use] extern crate anyhow;
#[macro_use] extern crate serenity;
#[macro_use] extern crate tokio;
