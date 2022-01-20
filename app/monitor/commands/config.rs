#[group]
#[prefixes("cfg", "config", "settings")]
#[summary="Commands for managing configuration details."]
#[commands(prefix)]
pub struct Config;


/// Change the command prefix to the supplied string.
#[command]
#[allowed_roles("Admin", "Foundation", "Kingsmen")]
pub async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
   // Send a message in the channel where the command was received.
   // This should never be an Err, so we can--in theory--safely discard
   // the result of this function call.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|e| {
         e.title("Settings: Prefix Update");
         e.description("This operation is not yet fully implemented.");

         e
      })
   }).await?;

   return Ok(());
}


// IMPORTS ////////////////////////////////////////////////////////////////////////////////////////

use automan::ShardManagerContainer;

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
      HelpOptions,
      help_commands,
      macros::{
         command,
         group,
         help,
      },
   },
   model::{
      channel::Message,
      prelude::UserId,
   },
};
