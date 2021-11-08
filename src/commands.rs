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
         });

         m
      }).await.unwrap();

      channels[0].send_message(&ctx.http, |m| {
         m.embed(|mut e| {
            e.title("Welcome!");
            e.description(&format!("@everyone, please welcome @{}#{} to the server!", member.user.name, member.user.id));

            e
         });

         m
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

/// Chat management commands and functionality.
pub mod chat;
pub use chat::CHAT_GROUP;

/// Chat game commands.
pub mod games;

/// General-use commands.
pub mod general;
pub use general::MY_HELP;
pub use general::GENERAL_GROUP;

/// Owner-specific commands.
pub mod owner;
pub use owner::OWNER_GROUP;
