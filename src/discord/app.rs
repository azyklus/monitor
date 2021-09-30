use anyhow::Result;

use serenity::{
   client::{
      Client,
      bridge::gateway::{
         GatewayIntents,
         ShardId,
         ShardManager,
      },
   },
   framework::standard::{
      StandardFramework,
   },
   prelude::*,
};

use std::{
   cell::RefCell,
   sync::Arc,
   io,
};

use tokio::sync::Mutex;

use crate::commands::{self, Handler};
use super::config::{self, DiscordConfig};

/// A container type is created for inserting into our [`Client`]'s data,
/// which allows for data to be accessible across all events and framework commands,
/// or anywhere else that has a copy of the data [`Arc`].
///
/// [`Client`]: serenity::client::Client
/// [`Arc`]: std::sync::Arc
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer
{
   type Value = Arc<Mutex<ShardManagerContainer>>;
}



/// Contains the main loop for the Discord bot.
pub struct Bot
{
   client: Client,
   config: DiscordConfig,
}

impl Bot
{
   /// Creates a new instance of the bot.
   pub async fn new() -> Result<Bot>
   {
      let fw = StandardFramework::new()
         .configure(|c| {
            c.prefix("mntr ")
         })
         .group(&commands::GENERAL_GROUP);

      let mut conf: DiscordConfig = DiscordConfig::default();
      conf = config::load(conf)?;

      let mut client: Client = Client::builder((&conf).token())
         .event_handler(Handler)
         .framework(fw)
         .await
         .expect("Error creating client");

      return Ok(Bot{
         config: conf,
         client: client,
      });
   }

   /// Returns a pointer to the Discord configuration.
   #[inline]
   pub fn config(&self) -> Arc<DiscordConfig> { Arc::new(self.config.clone()) }

   /// Runs the Discord bot.
   pub async fn run(&mut self) -> Result<()>
   {
      if let Err(e) = self.client.start().await {
         return Err(e.into());
      }

      return Ok(());
   }
}
