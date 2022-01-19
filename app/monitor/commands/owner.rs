/// Commands specific to Owner/Admin roles.
#[group]
#[owners_only]
#[only_in(guilds)]
#[commands(slowmode, stop)]
pub struct Owner;

/// Shuts down the bot.
///
/// Typically, this command should only be used for testing/maintenance.
#[command]
#[owners_only]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult
{
   let data = ctx.data.read().await;

   return if let Some(mngr) = data.get::<ShardManagerContainer>() {
      msg.reply(ctx, "Shutting down!").await?;
      mngr.lock().await.shutdown_all().await;

      Ok(())
   } else {
      msg.reply(ctx, "There was a problem getting the shard manager").await?;

      Ok(())
   };
}

/// Sets the slow mode rate for the channel in which the command is triggered.
///
///
/// # Examples
///
/// `mntr slow 10` > Sets the channel's slow mode rate to `10` seconds.
/// `mntr freeze 90` > Sets the channel's slow mode rate to `90` seconds.
#[command]
#[aliases("slow", "freeze")]
pub async fn slowmode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
   if let Ok(rate) = args.single::<u64>() {
      if let Err(e) = msg.channel_id.edit(&ctx.http, |c| {
         c.slow_mode_rate(rate)
      }).await {
         log::error!("an error occurred setting channel's slow mode rate");

         // Send a message in the channel where we received the command.
         // This should never return an Err, so we can theoretically safely
         // discard the result.
         let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
               e.title("Error!");

               e.author(|a| {
                  a.icon_url(&msg.author.avatar_url().unwrap());
                  a.name("The Monitor");

                  a
               });

               e.description("Failed to set this channel's slow mode rate.");

               e.footer(|f| {
                  f.text("Run the help command to see more information about this command.")
               });

               e
            })
         }).await?;
         return Err(e.into());
      } else {
         // Send a message in the channel where we received the command.
         // This should never return an Err, so we can theoretically safely
         // discard the result.
         let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
               e.title("Success!");

               e.author(|a| {
                  a.icon_url(&msg.author.avatar_url().unwrap());
                  a.name("The Monitor");

                  a
               });

               e.description(format!("Set this channel's slow mode rate to {} seconds", rate));

               e.footer(|f| {
                  f.text("Run the help command to see more information about this command.")
               });

               e
            })
         }).await?;
      }
   } else if let Some(Channel::Guild(channel)) = msg.channel_id.to_channel_cached(&ctx.cache).await {
      // Send a message in the channel where we received the command.
      // This should never return an Err, so we can theoretically safely
      // discard the result.
      let _ = msg.channel_id.send_message(&ctx.http, |m| {
         m.embed(|e| {
            e.title("Notice!");

            e.author(|a| {
               a.icon_url(&msg.author.avatar_url().unwrap());
               a.name("The Monitor");

               a
            });

            e.description(format!("Current slow mode rate is `{}` seconds.", channel.slow_mode_rate.unwrap_or(0)));

            e.footer(|f| {
               f.text("Run the help command to see more information about this command.")
            });

            e
         })
      }).await?;
   } else {
      // Send a message in the channel where we received the command.
      // This should never return an Err, so we can theoretically safely
      // discard the result.
      let _ = msg.channel_id.send_message(&ctx.http, |m| {
         m.embed(|e| {
            e.title("Error!");

            e.author(|a| {
               a.icon_url(&msg.author.avatar_url().unwrap());
               a.name("The Monitor");

               a
            });

            e.description("Failed to find the channel in cache.");

            e.footer(|f| {
               f.text("Run the help command to see more information about this command.")
            });

            e
         })
      }).await?;
   }

   return Ok(());
}



// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use automan::ShardManagerContainer;

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
      channel::{Channel, Message},
   },
};
