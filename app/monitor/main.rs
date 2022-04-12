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
#![allow(clippy::needless_return)]
#![feature(exclusive_range_pattern)]


// MAIN APPLICATION LOGIC ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

lazy_static! {
   /// The default config object.
   pub static ref DEFAULT_CONFIG: AppConfig = AppConfig::default();

   /// A global CONFIG object.
   pub static ref CONFIG: AppConfig = load_config(DEFAULT_CONFIG.clone()).unwrap();

   /// The main GIPHY client object.
   pub static ref GIPHY: GiphyBot = GiphyBot::new(&CONFIG.giphy).unwrap();
}


#[doc(hidden)]
#[tokio::main]
async fn main() -> Result<(), GenericError>
{
   if let Err(e) = Dispatch::new()
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
      .apply() {
      return Err(e.into());
   }


   let config = CONFIG.clone();
   let http = Http::new_with_token(&config.discord.token);

   // We will fetch your bot's owners and id
   let (owners, bot_id) = match http.get_current_application_info().await {
      Ok(info) => {
         let mut owners = HashSet::new();

         if let Some(team) = info.team {
            owners.insert(team.owner_user_id);
         } else {
            owners.insert(info.owner.id);
         }

         match http.get_current_user().await {
            Ok(bot_id) => (owners, bot_id.id),
            Err(why) => panic!("Could not access the bot id: {:?}", why),
         }
      }
      Err(why) => panic!("Could not access application info: {:?}", why),
   };

   // Configure Serenity's standard framework.
   let fw = StandardFramework::new()
      .configure(|c| {
         c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix(&config.discord.prefix)
            // In this case, if "," would be first, a message would never
            // be delimited at ", ", forcing you to trim your arguments if you
            // want to avoid whitespaces at the start of each.
            .delimiters(vec![", ", ","])
            // Sets the bot's owners. These will be used for commands that
            // are owners only.
            .owners(owners)
      })
      // Set a function to be called prior to each command execution. This
      // provides the context of the command, the message that was received,
      // and the full name of the command that will be called.
      //
      // Avoid using this to determine whether a specific command should be
      // executed. Instead, prefer using the `#[check]` macro which
      // gives you this functionality.
      //
      // **Note**: Async closures are unstable, you may use them in your
      // application if you are fine using nightly Rust.
      // If not, we need to provide the function identifiers to the
      // hook-functions (before, after, normal, ...).
      .before(hooks::before)
      // Similar to `before`, except will be called directly _after_
      // command execution.
      .after(hooks::after)
      // Set a function that's called whenever an attempted command-call's
      // command could not be found.
      .unrecognised_command(hooks::unknown)
      // Set a function that's called whenever a message is not a command.
      .normal_message(hooks::normal)
      // Set a function that's called whenever a command's execution didn't complete for one
      // reason or another. For example, when a user has exceeded a rate-limit or a command
      // can only be performed by the bot owner.
      .on_dispatch_error(hooks::dispatch_error)
      // Can't be used more than once per 5 seconds:
      .bucket("emoji", |b| b.delay(5))
      .await
      // Can't be used more than 2 times per 30 seconds, with a 5 second delay applying per channel.
      // Optionally `await_ratelimits` will delay until the command can be executed instead of
      // cancelling the command invocation.
      .bucket("complicated", |b| {
         b
            .limit(2)
            .time_span(30)
            .delay(5)
            // The target each bucket will apply to.
            .limit_for(LimitedFor::Channel)
            // The maximum amount of command invocations that can be delayed per target.
            // Setting this to 0 (default) will never await/delay commands and cancel the invocation.
            .await_ratelimits(1)
            // A function to call when a rate limit leads to a delay.
            .delay_action(hooks::delay_action)
      })
      .await
      .help(&commands::MY_HELP)
      .group(&commands::GENERAL_GROUP)
      .group(&commands::GIPHY_GROUP)
      .group(&commands::CHAT_GROUP)
      .group(&commands::CONFIG_GROUP)
      .group(&commands::FUN_GROUP)
      .group(&commands::OWNER_GROUP)
      .group(&commands::GAMES_GROUP)
      .group(&commands::TWITTER_GROUP)
      .group(&commands::XKCD_GROUP);

   let discord: DiscordBot = { automan::setup_discord(&config.discord, fw).await? };
   let giphy: GiphyBot = automan::setup_giphy(&config.giphy)?;
   let matrix: MatrixBot = automan::setup_matrix(&config.matrix)?;

   return automan::start(config, discord, giphy, matrix).await;
}


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
            e.description(&format!("Please welcome @{}#{} to the server!", member.user.name, member.user.id));

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

/// A container type is created for inserting into our [`Client`]'s data,
/// which allows for data to be accessible across all events and framework commands,
/// or anywhere else that has a copy of the data [`Arc`].
///
/// [`Client`]: serenity::client::Client
/// [`Arc`]: std::sync::Arc
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer
{
   type Value = Arc<Mutex<ShardManager>>;
}


// MODULES //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// `Bot` is implemented here.
pub mod bot;

/// Contains bot commands called on Discord or Matrix.
pub mod commands;

/// Serenity API hooks.
pub mod hooks;

/// Application services for handling tasks automatically.
pub mod services;


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use automan::{
   discord::{hooks, DiscordBot},
   errors::GenericError,
   gif::GiphyBot,
   matrix::MatrixBot,
   shared::*,
};

use serenity::{
   async_trait,
   client::EventHandler,
   framework::standard::{
      buckets::LimitedFor,
      StandardFramework,
   },
   http::Http,
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


// CRATE DEPENDENCIES ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern crate chrono;
extern crate fern;
extern crate log;
extern crate rand;
extern crate rillrate;
extern crate serde;
extern crate serde_json;
extern crate serenity;
extern crate tokio;
extern crate toml;
extern crate ulid;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
