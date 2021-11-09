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

// MODULES //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Chat management commands and functionality.
pub mod chat;
pub use chat::CHAT_GROUP;

/// Chat game commands.
pub mod games;
pub use games::GAMES_GROUP;

/// General-use commands.
pub mod general;
pub use general::MY_HELP;
pub use general::GENERAL_GROUP;

/// Owner-specific commands.
pub mod owner;
pub use owner::OWNER_GROUP;


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use serenity::{
   async_trait,
   client::EventHandler,
   model::{
      gateway::Ready,
      guild::Member,
      id::{ChannelId, GuildId},
   },
   prelude::*,
};

use std::collections::{HashMap, HashSet};
