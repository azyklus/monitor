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


// MAIN APPLICATION LOGIC ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[tokio::main]
async fn main() -> Result<(), GenericError>
{
   if let Err(e) = automan::setup_logging(LevelFilter::Debug, ".logfile") {
      return Err(e.into());
   }

   let mut config: AppConfig = AppConfig::default();
   config = automan::shared::load_config(config)?;

   let mut discord: DiscordBot = automan::setup_discord(&config.discord).await?;
   let mut matrix: MatrixBot = automan::setup_matrix(&config.matrix)?;

   return automan::start(config, discord, matrix).await;
}


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use automan::{
   discord::DiscordBot,
   errors::GenericError,
   matrix::MatrixBot,
   shared::AppConfig,
};

use log::LevelFilter;


// CRATE DEPENDENCIES ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern crate anyhow;
extern crate automan;
extern crate tokio;
