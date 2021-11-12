/// Commands specific to Owner/Admin roles.
#[group]
#[commands(stop)]
pub struct Owner;

/// Shuts down the bot.
///
/// Typically, this command should only be used for testing/maintenance.
#[command]
#[owners_only]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult
{
   let data = ctx.data.read().await;

   if let Some(mngr) = data.get::<ShardManagerContainer>() {
      msg.reply(ctx, "Shutting down!").await?;
      mngr.lock().await.shutdown_all().await;

      return Ok(());
   } else {
      msg.reply(ctx, "There was a problem getting the shard manager").await?;

      return Ok(());
   }

   return Ok(());
}


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use automan::ShardManagerContainer;

use serenity::{
   client::Context,
   framework::standard::{
      CommandResult,
      macros::{
         command,
         group,
      },
   },
   model::channel::Message,
};
