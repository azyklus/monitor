use crate::commands::CommandCounter;

use serenity::{
   client::{
      Client,
      bridge::gateway::{
         GatewayIntents,
         ShardId,
         ShardManager,
      },
   },
   framework::standard::{
      CommandResult,
      DispatchError,
      StandardFramework,
      macros::hook,
   },
   http::Http,
   model::{
      channel::Message,
      prelude::UserId,
   },
   prelude::*,
};

#[hook]
pub async fn before(ctx: &Context, msg: &Message, name: &str) -> bool
{
   trace!("Got command `{}` by user `{}`", name, msg.author.name);

   // Increment the number of times that this command has been run by one.
   // If the command's name is not currently in the counter, add a default
   // value of zero.
   let mut data = ctx.data.write().await;
   let counter = data.get_mut::<CommandCounter>().expect("expected CommandCounter to be in TypeMap");
   let entry = counter.entry(name.to_string()).or_insert(0);
   *entry += 1;


   return true; // If this function returns false, command processing does not happen.
}

#[hook]
pub async fn after(_ctx: &Context, _msg: &Message, name: &str, res: CommandResult)
{
   match res {
      Ok(()) => trace!("Processed command `{}`", name),
      Err(why) => error!("Command `{}` returned error: {:?}", name, why),
   }
}

#[hook]
pub async fn unknown(_ctx: &Context, msg: &Message, name: &str)
{
   error!("Could not find command called `{}`", name);
}

#[hook]
pub async fn normal(_ctx: &Context, msg: &Message)
{
   warn!("Message is not a command: `{}`", msg.content);
}

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message)
{
   // NOTE: May want to handle a Discord ratelimit if this fails.
   let _ = msg.react(ctx, '⏱').await;
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError)
{
   if let DispatchError::Ratelimited(info) = error {
      if info.is_first_try {
         let _ = msg
            .channel_id
            .say(&ctx.http, &format!("Try again in {} seconds", info.as_secs()))
            .await;
      }
   }
}
