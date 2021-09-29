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

#[group]
#[commands(ping)]
pub struct General;

/// Replies with "Pong!"
///
/// Mostly just a test command.
#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult
{
   msg.reply(ctx, "Pong!").await?;

   return Ok(());
}
