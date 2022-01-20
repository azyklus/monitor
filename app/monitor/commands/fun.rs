/// Commands used for general amusement.
#[group]
#[summary="General amusement."]
#[commands(hug, slap)]
pub struct Fun;


/// Give someone a hug!
#[command]
pub async fn hug(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
   let mut rnd       = rngs::OsRng;
   let num: usize    = rnd.gen_range(1..=25) as usize;
   let gif: Vec<Gif> = (&GIPHY).search("hug").await.expect("couldn't retrieve GIF");
   let par: String   = args.single().expect("invalid arguments");

   if let Some(mem) = &msg.member {
      if let Some(usr) = mem.user.clone() {
         // Send a message in the channel where the command was received.
         // This should never be an Err, so we can--in theory--safely discard
         // the result of this function call.
         let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
               e.description(format!("{} hugged {}!", usr.name, par));
               e.image(&gif[num].url);

               e
            })
         }).await?;
      } else {
         msg.reply(ctx, "Couldn't hug your special someone! Sorry!").await?;
      }
   }

   return Ok(());
}

/// Slap someone!
#[command]
pub async fn slap(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
   let mut rnd       = rngs::OsRng;
   let num: usize    = rnd.gen_range(1..=25) as usize;
   let gif: Vec<Gif> = (&GIPHY).search("slap").await.expect("couldn't retrieve GIF");
   let par: String   = args.single().expect("invalid arguments");

   if let Some(mem) = &msg.member {
      if let Some(usr) = mem.user.clone() {
         // Send a message in the channel where the command was received.
         // This should never be an Err, so we can--in theory--safely discard
         // the result of this function call.
         let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
               e.description(format!("{} slapped {}!", usr.name, par));
               e.image(&gif[num].url);

               e
            })
         }).await?;
      } else {
         msg.reply(ctx, "Couldn't slap anyone.").await?;
      }
   }

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
      channel::Message,
      user::User,
   },
};
