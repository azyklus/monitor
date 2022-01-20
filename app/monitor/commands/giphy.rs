/// A group for commands related to the GIPHY API.
#[group]
#[prefixes("giphy")]
#[summary="Get GIFs from GIPHY."]
#[commands(trending, random, search)]
pub struct Giphy;


/// Search for GIFs.
#[command]
pub async fn search(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
   let mut rnd = rngs::OsRng;
   let num = rnd.gen_range(1..=25) as usize;

   let text: String   = args.single().expect("expected a string");
   let gifs: Vec<Gif> = (&GIPHY).search(text.as_str()).await.expect("failed to retrieve GIFs");

   // Send a message in the channel where the command was sent.
   // This function should never Err, so we may safely discard
   // the result of the call.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.content(&gifs[num].url)
   }).await?;

   //   for i in 0..=i32::MAX {
   //      if i == i32::MAX {
   //         msg.delete(&ctx.http).await.expect("failed to delete message");
   //         break;
   //      } else {
   //         continue;
   //      }
   //   }
   //
   // THE ABOVE CODE IS RETARDED.

   return Ok(());
}

/// Gets a random GIF and sends it in a Discord message.
#[command]
pub async fn random(ctx: &Context, msg: &Message) -> CommandResult
{
   let gif: Gif = (&GIPHY).random().await.expect("failed to retrieve GIF");

   // Send a message in the channel where we received the command.
   // This should never return an Err, so we can theoretically safely
   // discard the result.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.content(&gif.url)
   }).await?;

   return Ok(());
}

/// Gets a random GIF from the 'Trending' section of GIPHY
/// and sends it in a Discord message.
#[command]
pub async fn trending(ctx: &Context, msg: &Message) -> CommandResult
{
   let mut rnd = rngs::OsRng;
   let num = rnd.gen_range(0..25) as usize;

   let gifs: Vec<Gif> = (&GIPHY).trending().await.unwrap();

   // Send a message in the channel where we received the command.
   // This should never return an Err, so we can theoretically safely
   // discard the result.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.content(&gifs[num].url)
   }).await?;

   return Ok(());
}


// IMPORTS ////////////////////////////////////////////////////////////////////////////////////////

use crate::GIPHY;

use giphy::v1::Gif;

use rand::{Rng, rngs};

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
