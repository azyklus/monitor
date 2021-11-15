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



/// Contains the main loop for the Discord bot.
pub struct DiscordBot
{
   client: Client,
   config: DiscordConfig,
}

impl DiscordBot
{
   /// Creates a new instance of the Discord bot.
   pub async fn new(config: &DiscordConfig, fw: StandardFramework) -> Result<DiscordBot>
   {
      let http = Http::new_with_token(config.token.as_str());

      let (owners, bot_id) = match http.get_current_application_info().await {
         Ok(ini) => {
            let mut owners = HashSet::new();
            if let Some(team) = ini.team {
               owners.insert(team.owner_user_id);
            } else {
               owners.insert(ini.owner.id);
            }

            match http.get_current_user().await {
               Ok(bot_id) => (owners, bot_id.id),
               Err(why) => panic!("Could not access the DiscordBot id: {:?}", why),
            }
         },
         Err(why) => panic!("Could not access application info: {:?}", why),
      };

      let mut client: Client = Client::builder(&config.token)
         .event_handler(Handler)
         .framework(fw)
         // For this to run properly, the "Presence Intent" and "Server Members Intent"
         // options need to be enabled.
         // These are needed so the `required_permissions` macro works on the commands that need to
         // use it.
         // You will need to enable these 2 options on the DiscordBot application, and possibly wait up to 5
         // minutes.
         .intents(GatewayIntents::all())
         .await
         .expect("Error creating client");

      return Ok(DiscordBot{
         config: config.clone(),
         client,
      });
   }

   /// Returns a pointer to the Discord configuration.
   #[inline]
   pub fn config(&self) -> Arc<DiscordConfig> { Arc::new(self.config.clone()) }

   /// Runs the Discord DiscordBot.
   pub async fn run(&mut self) -> Result<()>
   {
      {
         let mut data = self.client.data.write().await;
         data.insert::<CommandCounter>(HashMap::default());
         data.insert::<ShardManagerContainer>(Arc::clone(&self.client.shard_manager));
      }

      if let Err(e) = self.client.start().await {
         return Err(e.into());
      }

      return Ok(());
   }
}

use anyhow::Result;

use crate::{CommandCounter, Handler};

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
   http::Http,
   prelude::*,
};

use std::{
   cell::RefCell,
   collections::{HashMap, HashSet},
   sync::Arc,
   io,
};

use tokio::sync::Mutex;

use super::config::{self, DiscordConfig};
