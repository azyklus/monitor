#[group]
#[commands(delete, wipe)]
pub struct Chat;

/// Deletes the specified number of messages, up to 99.
///
///
/// # Examples
///
/// `mntr rm 10` > Deletes 10 messages, plus the message that contained the command.
/// `mntr del 92` > Deletes 93 messages.
/// `mntr delete 68` > Deletes 69 messages. Nice.
///
///
/// # Notes
///
/// This command cannot be used to delete messages that are greater than two weeks
/// of age. Those will have to be deleted manually.
#[command]
#[aliases("del", "rm")]
pub async fn delete(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
   let n: u64 = args.parse()?;
   if n < 2 {
      log::warn!("an invalid integer value was supplied to the 'delete' command");
      let _ = msg.channel_id.send_message(&ctx.http, |m| {
         m.embed(|e| {
            e.title("Error!");

            e.author(|a| {
               a.icon_url(&msg.author.avatar_url().unwrap());
               a.name("The Monitor");

               a
            });

            e.description("There was an invalid integer passed to the `delete` command.");

            e.footer(|f| {
               f.text("Run the help command and append the name `delete` to see detailed information about this command.");

               f
            });

            e
         });

         m
      }).await;
   }

   let msgs = msg.channel_id.messages(&ctx.http, |r| {
      r.before(msg.id).limit(n)
   }).await?;

   let mut msg_ids: Vec<MessageId> = vec![];
   for m in msgs.iter() {
      let id = m.id;
      msg_ids.push(id);
   }

   if let Err(e) = msg.channel_id.delete_messages(&ctx.http, &msg_ids).await {
      log::error!("encountered an error whilst trying to delete the messages");
      return Err(e.into());
   }

   if let Err(e) = msg.delete(&ctx.http).await {
      log::error!("encountered an error trying to delete the command message");
      return Err(e.into());
   }

   let _ = msg.reply(&ctx.http, format!("Successfully deleted {} messages!", &msg_ids.len()));

   return Ok(());
}

/// Deletes the last hundred messages from the channel.
///
///
/// # Examples
///
/// `mntr wipe` > This will delete 99 messages, plus the message that triggered the
/// command.
///
///
/// # Notes
///
/// This command cannot be used to delete messages that are greater than two weeks
/// of age as Discord's API forbids it. Those messages will have to be manually removed.
#[command]
#[aliases("clean", "sanitize", "rmr")]
pub async fn wipe(ctx: &Context, msg: &Message) -> CommandResult
{
   let msgs = msg.channel_id.messages(&ctx.http, |r| {
      r.before(msg.id).limit(100)
   }).await?;

   let mut msg_ids: Vec<MessageId> = vec![];
   for m in msgs.iter() {
      let id = m.id;
      msg_ids.push(id);
   }

   if let Err(e) = msg.channel_id.delete_messages(&ctx.http, msg_ids).await {
      log::error!("encountered an error whilst trying to delete the messages");
      return Err(e.into());
   }

   return Ok(());
}

// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::ShardManagerContainer;

use std::collections::HashSet;

use serenity::{
   client::{
      Context,
      bridge::gateway::ShardId,
   },
   framework::standard::{
      Args,
      CommandGroup,
      CommandResult,
      help_commands,
      macros::{
         command,
         group,
      },
   },
   model::{
      channel::Message,
      prelude::{MessageId,UserId},
   },
};
