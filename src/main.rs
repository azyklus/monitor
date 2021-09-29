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
#![warn(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![feature(path_try_exists)]

use self::{
   discord::Bot,
   errors::{GenericError, OOBError},
};

use std::{
   error::Error,
   thread::{self, JoinHandle},
};

use tokio::runtime;

// MAIN APPLICATION LOGIC ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The maximum number of threads allowed to be running simultaneously.
pub const MAX_THREADS: usize = 3;

#[doc(hidden)]
#[tokio::main]
async fn main() -> Result<(), GenericError>
{
   let mut children = vec![];
   let mut discord: Bot = discord::setup().await?;

   let child1: JoinHandle<_> = thread::spawn(move || {
      runtime::Builder::new_multi_thread()
         .enable_all()
         .build()
         .unwrap()
         .block_on(async {
            let _ = discord.run().await;
         });
   });

   if children.len() > MAX_THREADS {
      return Err(OOBError.into());
   }

   if children.len() <= MAX_THREADS - 2 {
      children.push(child1);
   } else if children.len() == MAX_THREADS - 2 {
   } else if children.len() == MAX_THREADS - 1 {
   }

   for child in children {
      let _ = child.join();
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
