#![crate_name = "monitor"]
#![crate_type = "bin"]
#![deny(clippy::all)]
#![warn(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![feature(path_try_exists)]

pub use self::{
   commands::CommandCounter,
   discord::app::ShardManagerContainer,
};

use self::errors::{GenericError, OOBError};

use std::{
   error::Error,
};

use tokio::task::JoinHandle;

// MAIN APPLICATION LOGIC ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The maximum number of threads allowed to be running simultaneously.
pub const MAX_THREADS: usize = 3;

#[doc(hidden)]
#[tokio::main]
async fn main() -> Result<(), GenericError>
{
   let mut childs = vec![];
   let mut dis: discord::Bot = discord::setup().await?;

   let child1: JoinHandle<_> = tokio::spawn(async move {
      dis.run().await;
   });

   match childs.len() {
      MAX_THREADS => return Err(OOBError.into()),
      0 => childs.push(child1),
      1 => {},
      2 => {},
      _ => return Err(OOBError.into()),
   }

   for child in childs {
      let _ = child.await?;
   }

   return Ok(());
}

// CRATE MODULES ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains bot commands called on Discord or Matrix.
///
/// Commands are prefixed with `mntr` and can be enumerated via
/// the `mntr help` command.
pub mod commands;

/// Contains the main logic relating to the Discord bot.
pub mod discord;

/// Contains error types.
pub mod errors;

/// Contains the Matrix bot.
pub mod matrix;

// CRATE DEPENDENCIES ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern crate anyhow;
extern crate chrono;
extern crate fern;
extern crate lazy_static;
extern crate serde;
extern crate serenity;
extern crate tokio;
extern crate toml;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
