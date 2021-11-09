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
pub struct Bot
{
   client: Client,
   config: DiscordConfig,
}

impl Bot
{
   /// Creates a new instance of the bot.
   pub async fn new(config: &DiscordConfig) -> Result<Bot>
   {
      let http = Http::new_with_token(config.token().as_str());

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
               Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
         },
         Err(why) => panic!("Could not access application info: {:?}", why),
      };

      let fw = StandardFramework::new()
         .configure(|c| {
            c
               .prefix("mntr ")
               .on_mention(Some(bot_id))
               .with_whitespace(true)
               .delimiters(vec![", ", ","])
               .owners(owners)
         })
         .before(hooks::before)
         .after(hooks::after)
         .unrecognised_command(hooks::unknown)
         .normal_message(hooks::normal)
         .on_dispatch_error(hooks::dispatch_error)
         .help(&commands::MY_HELP)
         .group(&commands::CHAT_GROUP)
         .group(&commands::GAMES_GROUP)
         .group(&commands::GENERAL_GROUP)
         .group(&commands::OWNER_GROUP);

      let mut client: Client = Client::builder(&config.token())
         .event_handler(Handler)
         .framework(fw)
         // For this to run properly, the "Presence Intent" and "Server Members Intent"
         // options need to be enabled.
         // These are needed so the `required_permissions` macro works on the commands that need to
         // use it.
         // You will need to enable these 2 options on the bot application, and possibly wait up to 5
         // minutes.
         .intents(GatewayIntents::all())
         .await
         .expect("Error creating client");

      return Ok(Bot{
         config: config.clone(),
         client,
      });
   }

   /// Returns a pointer to the Discord configuration.
   #[inline]
   pub fn config(&self) -> Arc<DiscordConfig> { Arc::new(self.config.clone()) }

   /// Runs the Discord bot.
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

use crate::commands::CommandCounter;

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

use crate::commands::{self, Handler};
use super::config::{self, DiscordConfig};

#[doc(hidden)]
pub mod hooks;
