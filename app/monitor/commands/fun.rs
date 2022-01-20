#[group]
#[summary="General amusement."]
#[commands(hug, slap)]
pub struct Fun;


/// Give someone a hug!
#[command]
pub async fn hug(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
   // Send a message in the channel where the command was received.
   // This should never be an Err, so we can--in theory--safely discard
   // the result of this function call.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|e| {
         e.title("Function: Hug");
         e.description("This operation is not yet fully implemented.");

         e
      })
   }).await?;

   return Ok(());
}

/// Slap someone!
#[command]
pub async fn slap(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
   // Send a message in the channel where the command was received.
   // This should never be an Err, so we can--in theory--safely discard
   // the result of this function call.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|e| {
         e.title("Function: Slap");
         e.description("This operation is not yet fully implemented.");

         e
      })
   }).await?;

   return Ok(());
}


// IMPORTS ////////////////////////////////////////////////////////////////////////////////////////

use serenity::{
   client::{
      Context,
   },
   framework::standard::{
      Args,
      CommandResult,
      macros::{
         command,
         group,
      },
   },
   model::{
      channel::Message,
   },
};
