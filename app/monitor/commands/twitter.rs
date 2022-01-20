#[group]
#[prefixes("twitter", "tw")]
#[summary="Do stuff with Twitter."]
#[commands(auth, tweets)]
pub struct Twitter;


/// Allow the bot to see your Tweets.
#[command]
pub async fn auth(ctx: &Context, msg: &Message) -> CommandResult
{
   // Send a message in the channel where the command was received.
   // This should never be an Err, so we can--in theory--safely discard
   // the result of this function call.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|e| {
         e.title("Twitter: Account Authentication");
         e.description("This operation is not yet fully implemented.");

         e
      })
   }).await?;

   return Ok(());
}

/// Get a user's Tweets, if available.
#[command]
pub async fn tweets(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
   // Send a message in the channel where the command was received.
   // This should never be an Err, so we can--in theory--safely discard
   // the result of this function call.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|e| {
         e.title("Twitter: Post Retrieval");
         e.description("This operation is not yet fully implemented.");

         e
      })
   }).await?;

   return Ok(());
}


// IMPORTS ////////////////////////////////////////////////////////////////////////////////////////

use serde_json::de;

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
      channel::{Message},
   },
};
