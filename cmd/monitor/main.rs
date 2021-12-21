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
   if let Err(e) = automan::setup_logging(LevelFilter::Debug, ".logfile") {
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
         b.limit(2)
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
      .group(&commands::CHAT_GROUP)
      .group(&commands::OWNER_GROUP)
      .group(&commands::GAMES_GROUP)
      .group(&commands::XKCD_GROUP);

   let mut discord: DiscordBot = automan::setup_discord(&config.discord, fw).await?;
   let mut giphy: GiphyBot = automan::setup_giphy(&config.gif)?;
   let mut matrix: MatrixBot = automan::setup_matrix(&config.matrix)?;

   return automan::start(config, discord, giphy, matrix).await;
}

// MODULES //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains bot commands called on Discord or Matrix.
///
/// Commands are prefixed with `mntr` and can be enumerated via
/// the `mntr help` command.
pub mod commands;

// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use automan::{
   discord::{hooks, DiscordBot},
   errors::GenericError,
   gif::{config::GiphyConfig, GiphyBot},
   matrix::MatrixBot,
   shared::*,
   CommandCounter,
};

use serenity::{
   client::{
      bridge::gateway::{GatewayIntents, ShardId, ShardManager},
      Client,
   },
   framework::standard::{
      buckets::{LimitedFor, RevertBucket},
      macros::hook,
      CommandResult,
      DispatchError,
      StandardFramework
   },
   http::Http,
   model::{channel::Message, prelude::UserId},
   prelude::*,
};

use std::collections::{HashMap, HashSet};

use log::LevelFilter;

// CRATE DEPENDENCIES ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern crate anyhow;
extern crate automan;
extern crate serenity;
extern crate tokio;

#[macro_use]
extern crate lazy_static;
