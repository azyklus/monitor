//! `automan` is a support library for the Monitor program.
//!
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
#![crate_name = "automan"]
#![crate_type = "lib"]
#![deny(clippy::all)]
#![warn(missing_docs)]
#![allow(unused)]
#![allow(clippy::needless_return)]
#![allow(dead_code)]
#![feature(decl_macro)]
#![feature(path_try_exists)]

/// The maximum number of threads allowed to remain activate at once.
pub const MAX_THREADS: usize = 3;

/// An asynchronous event handler.
///
/// An instance of our `Handler` will watch for events defined in the [`EventHandler`] trait impl.
///
/// `guild_member_addition` is an event triggered when a [`User`] joins a [`Guild`] that the bot is
/// party to. When this event is triggered, a message is sent to the first channel in the guild welcoming
/// the user and the user receives a welcome DM with instructions on how best to proceed.
///
/// `ready` is triggered when the bot finishes connecting to the Discord gateway.
///
/// [`EventHandler`]: serenity::client::EventHandler
/// [`User`]: serenity::model::user::User
/// [`Guild`]: serenity::model::guild::Guild
pub struct Handler;

#[async_trait]
impl EventHandler for Handler
{
   async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, member: Member)
   {
      let mut channels: Vec<ChannelId> = vec![];
      let http = ctx.http.clone();

      for channel in http.as_ref().get_channels(guild_id.0).await.unwrap() {
         channels.push(channel.id);
      }

      member.user.dm(&ctx.http, |m| {
         m.embed(|mut e| {
            e.title("Welcome aboard!");
            e.description("You have successfully joined the Narwhals fan server!");

            e
         })
      }).await.unwrap();

      channels[0].send_message(&ctx.http, |m| {
         m.embed(|mut e| {
            e.title("Welcome!");
            e.description(&format!("@everyone, please welcome @{}#{} to the server!", member.user.name, member.user.id));

            e
         })
      }).await.unwrap();

      log::info!("{} joined the guild at {:?}.", member.user.name, member.joined_at);
   }

   async fn ready(&self, _: Context, ready: Ready)
   {
      log::info!("{} is connected!", ready.user.name);
   }
}

#[doc(hidden)]
pub struct CommandCounter;

impl TypeMapKey for CommandCounter
{
   type Value = HashMap<String, u64>;
}


/// Set up logging functionality for the Monitor application.
pub fn setup_logging(level: LevelFilter, logfile: &str) -> Result<(), fern::InitError>
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
      .level(level)
      .chain(std::io::stdout())
      .chain(fern::log_file(logfile)?)
      .apply()?;

   return Ok(());
}

/// Sets up the Discord bot.
pub async fn setup_discord(config: &DiscordConfig, fw: StandardFramework) -> Result<DiscordBot, GenericError>
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

/// Sets up the Giphy bot.
pub fn setup_giphy(config: &GiphyConfig) -> Result<GiphyBot, GenericError>
{
   let mut bot: GiphyBot = GiphyBot::new(config).unwrap();

   return Ok(bot);
}

/// Sets up the Matrix bot.
pub fn setup_matrix(config: &MatrixConfig) -> Result<MatrixBot, GenericError>
{
   let mut bot: MatrixBot = MatrixBot::new(config).unwrap();

   return Ok(bot);
}

/// STARTs the bot.
pub async fn start(
    options: AppConfig,
    mut discord: DiscordBot,
    mut giphy: GiphyBot,
    mut matrix: MatrixBot
) -> Result<(), GenericError>
{
   log::info!("THE MONITOR");
   log::info!("---------------");
   log::info!("Version: v0.1.0");
   log::info!("Support: AUTOMAN-0.0.1");
   log::info!("Identifier: {}", options.id);

   log::warn!("---");
   log::warn!("PLEASE NOTE THAT THIS IS A VERY EARLY PRE-RELEASE VERSION OF THE PROGRAM.");
   log::warn!("ALMOST EVERYTHING IN THIS APPLICATION IS SUBJECT TO CHANGE IN THE FUTURE.");
   log::warn!("---");

   let mut childs = vec![];

   let child1: JoinHandle<_> = tokio::spawn(async move {
      discord.run().await;
   });

   let child2: JoinHandle<_> = tokio::spawn(async move {
      matrix.run().await;
   });

   match childs.len() {
      MAX_THREADS => return Err(OOBError.into()),
      0 => childs.push(child1),
      1 => childs.push(child2),
      2 => {}
      _ => return Err(OOBError.into()),
   }

   for child in childs {
      let _ = child.await?;
   }

   return Ok(());
}

// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub use crate::{
   discord::app::ShardManagerContainer,
   errors::GenericError,
   shared::AppConfig,
};

use self::{
   discord::{
      DiscordBot,
      config::DiscordConfig,
   },
   errors::*,
   gif::{
      GiphyBot,
      config::GiphyConfig,
   },
   matrix::{
      MatrixBot,
      config::MatrixConfig,
   },
};

use serenity::{
   async_trait,
   client::EventHandler,
   framework::StandardFramework,
   model::{
      gateway::Ready,
      guild::Member,
      id::{ChannelId, GuildId},
   },
   prelude::*,
};

use std::collections::{HashMap, HashSet};


use std::error::Error;
use tokio::task::JoinHandle;

use chrono::Utc;
use fern::Dispatch;
use log::LevelFilter;

use ulid::Ulid;

// CRATE MODULES ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains an implementation of a Discord client via [Serenity].
///
/// [Serenity]: https://github.com/serenity-rs/serenity
pub mod discord;

/// Contains error types.
pub mod errors;

/// Contains light implementations of various chat games.
pub mod games;

/// Contains a GIPHY client implementation.
pub mod gif;

/// Macros for basic utilities in the Monitor.
pub mod macros;

/// Contains an implementation of a Matrix bot and client.
pub mod matrix;

/// Contains globally accessible configuration details.
pub mod shared;

// DEPENDENCIES /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern crate chrono;
extern crate fern;
extern crate rand;
extern crate rillrate;
extern crate serde;
extern crate serenity;
extern crate tokio;
extern crate toml;
extern crate ulid;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
