/// A group for commands related to the GIPHY API.
#[group]
#[prefixes("giphy")]
#[summary="Get GIFs from GIPHY."]
#[commands(trending, random)]
pub struct Giphy;


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
