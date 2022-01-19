/// The Xkcd commands fetch comics straight from Xkcd itself.
///
/// - `xkcd random` will get a random comic.
/// - `xkcd latest` will get the latest comic.
/// - `xkcd select` will get the specified comic. (This command takes an integer argument.)
///
/// 
/// # Notes
/// 
/// A `search` command has not yet been implemented but may be at some point in the future.
/// Right now, the three base commands are the most important for the Xkcd group.
#[group]
#[prefixes("xkcd")]
#[summary="Commands for getting XKCD comics."]
#[commands(latest, random, select)]
pub struct Xkcd;

/// Represents an Xkcd comic.
#[derive(Debug, Deserialize)]
pub struct XkcdComic
{
   pub month: String,
   pub num: usize,
   pub link: String,
   pub year: String,
   pub news: String,
   pub safe_title: String,
   pub alt: String,
   pub img: String,
   pub title: String,
   pub day: String,
}

/// Gets a random comic from Xkcd.
#[command]
pub async fn random(ctx: &Context, msg: &Message) -> CommandResult
{
   let mut rnd = rngs::OsRng;
   let num: usize = {
      // Send a request to XKCD's JSON web API to get the latest comic.
      let bod1 = reqwest::get("https://xkcd.com/info.0.json")
         .await
         .expect("failed to retrieve the comic from XKCD")
         // Separate the body from the rest of the HTTP request.
         .text()
         .await
         .expect("failed to separate the body of the request");

      // Deserialize our received JSON into our comic structure.
      let comic1: XkcdComic = de::from_str(bod1.as_str()).expect("failed to deserialize JSON data");

      // Generate our number.
      rnd.gen_range(1..=comic1.num) as usize
   };

   // Send a request to XKCD's JSON web API to get a random comic,
   // using the random number generated above.
   let url2 = format!("https://xkcd.com/{}/info.0.json", num);
   let bod2 = reqwest::get(url2)
      .await
      .expect("failed to retrieve the comic from XKCD")
      // Separate the body from the rest of the HTTP request.
      .text()
      .await
      .expect("failed to separate the body from the request");

   // Deserialize our received JSON data into our comic structure.
   let comic2: XkcdComic = de::from_str(bod2.as_str()).expect("failed to deserialize JSON data");

   // Send a message in the channel where we received the command.
   // This should never return an Err, so we may theoretically safely
   // discard the result of this function call.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.content(&comic2.img)
   }).await?;

   return Ok(());
}

/// Gets the latest comic posted to Xkcd.
#[command]
pub async fn latest(ctx: &Context, msg: &Message) -> CommandResult
{
   let bod = reqwest::get("https://xkcd.com/info.0.json")
      .await
      .expect("failed to retrieve the comic from XKCD")
      .text()
      .await
      .expect("failed to separate the body of the request");
    
   // Deserialize our received JSON into our comic structure.
   let comic: XkcdComic = de::from_str(bod.as_str()).expect("failed to deserialize JSON data");

   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.content(&comic.img)
   }).await?;

    return Ok(());
}

/// Gets a specific comic from XKCD.
///
/// 
/// # Usage
///
/// - `mntr xkcd select 4` gets `https://xkcd.com/4`
#[command]
pub async fn select(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
   let param = args.single::<usize>().expect("expected a single integer");
    
   // Request the comic from XKCD.
   let url = format!("https://xkcd.com/{}/info.0.json", param);
   let bod = reqwest::get(url)
      .await
      .expect("failed to retrieve comic from XKCD")
      // Separate the body from the rest of the HTTP request.
      .text()
      .await
      .expect("failed to separate body from the request");

   // Deserialize our received JSON into our comic structure.
   let comic: XkcdComic = de::from_str(bod.as_str()).expect("failed to serialize JSON data");

   // Send a message in the channel where we received the command.
   // This should never return an Err, so we can theoretically safely
   // discard the result.
   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.content(&comic.img)
   }).await?;

   return Ok(());
}


// IMPORTS ////////////////////////////////////////////////////////////////////////////////////////

use rand::{Rng, rngs};

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
