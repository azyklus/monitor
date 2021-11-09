//! The Monitor is a simple Discord bot that hangs out in the [Narwhals] server.
//! The bot offers several chat games as well as tools to automatically perform certain
//! administrative tasks in order to better keep the server organized.
//!
//! You can add the bot to your own server [here], or you can build your own by following
//! [these instructions].
//!
//! [Narwhals]: https://discord.gg/GyXtwnBWne
//! [here]: https://discord.com/api/oauth2/authorize?client_id=817894435299262516&permissions=8&redirect_uri=https%3A%2F%2Fdiscord.com%2Fapi%2Foauth2%2Fauthorize%3Fclient_id%3D817894435299262516%26scope%3Dapplications.commands&scope=bot
//! [these instructions]: https://github.com/mnimi/monitor/#installation
#![crate_name = "monitor"]
#![crate_type = "bin"]
#![deny(clippy::all)]
#![allow(unused)]
#![allow(clippy::needless_return)]
#![allow(dead_code)]
#![feature(path_try_exists)]

// MAIN APPLICATION LOGIC ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The maximum number of threads allowed to be running simultaneously.
pub const MAX_THREADS: usize = 3;

/// Set up logging functionality for the Monitor application.
fn setup_logging() -> Result<(), fern::InitError>
{
   Dispatch::new()
      .format(|out, message, record| {
         out.finish(format_args!(
            "{}[{}][{}] {}",
            Utc::now().format("[%Y-%m-%d] [%HH:%Mm:%Ss]"),
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

#[doc(hidden)]
#[tokio::main]
async fn main() -> Result<(), GenericError>
{
   let app_id: String = GLOBAL_CONFIG.id();

   if let Err(e) = setup_logging() {
      return Err(e.into());
   }

   log::info!("Application: MONITOR");
   log::info!("Version: v0.1.0");
   log::info!("Support: AUTOMAN-0.0.1");
   log::info!("Application ID: {}", app_id);

   log::warn!("---");
   log::warn!("PLEASE NOTE THAT THIS IS A VERY EARLY PRE-RELEASE VERSION OF THE PROGRAM.");
   log::warn!("ALMOST EVERYTHING IN THIS APPLICATION IS SUBJECT TO CHANGE IN THE FUTURE.");
   log::warn!("---");

   let mut childs = vec![];
   let mut discord: discord::Bot = discord::setup(GLOBAL_CONFIG.discord.clone()).await?;

   let child1: JoinHandle<_> = tokio::spawn(async move { discord.run().await; });

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


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub use self::{
   commands::CommandCounter,
   discord::app::ShardManagerContainer,
   shared::AppConfig,
};

use automan::errors::{GenericError, OOBError};
use std::error::Error;
use tokio::task::JoinHandle;

use chrono::Utc;
use fern::Dispatch;
use log::LevelFilter;

use ulid::Ulid;


// CRATE MODULES ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains bot commands called on Discord or Matrix.
///
/// Commands are prefixed with `mntr` and can be enumerated via
/// the `mntr help` command.
pub mod commands;

/// Contains the main logic relating to the Discord bot.
pub mod discord;

/// Contains the Matrix bot.
pub mod matrix;

/// Contains shared application data for global access.
pub mod shared;
use shared::GLOBAL_CONFIG;

// CRATE DEPENDENCIES ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern crate anyhow;
extern crate automan;
extern crate chrono;
extern crate fern;
#[macro_use]
extern crate lazy_static;
extern crate rillrate;
extern crate serde;
extern crate serenity;
extern crate tokio;
extern crate toml;
extern crate ulid;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
