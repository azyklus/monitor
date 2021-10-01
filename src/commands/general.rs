use crate::ShardManagerContainer;

use std::collections::HashSet;

use serenity::{
   client::{
      Context,
      bridge::gateway::ShardId,
   },
   framework::standard::{
      Args,
      CommandGroup,
      CommandResult,
      HelpOptions,
      help_commands,
      macros::{
         command,
         group,
         help,
      },
   },
   model::{
      channel::Message,
      prelude::UserId,
   },
};

#[group]
#[commands(ping)]
pub struct General;

/// Shows the latency of the bot (in milliseconds).
#[command]
#[aliases("latency", "pong")]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult
{
   let data = ctx.data.read().await;

   let smgr = match data.get::<ShardManagerContainer>() {
      Some(v) => v,
      None => {
         msg.reply(ctx, "There was a problem getting the shard manager container.").await?;

         return Ok(());
      }
   };

   let mgr = smgr.lock().await;
   let rnnrs = mgr.runners.lock().await;

   let rnnr = match rnnrs.get(&ShardId(ctx.shard_id)) {
      Some(rnnr) => rnnr,
      None => {
         msg.reply(ctx, "No shard found!").await?;

         return Ok(());
      }
   };

   msg.reply(ctx, &format!("The shard's latency is {:?}", rnnr.latency.unwrap())).await?;

   return Ok(());
}

/// The Serenity framework provides us with two built-in help commands,
/// but we can use a custom help command that forwards the behaviour to
/// either of them.
///
/// We apply several attributes to this function, as listed here:
/// - individual_command_tip > This replaces information that a user passes as a command-name argument, 
/// granting specific information about it.
/// - command_not_found_text > The information presented when a supplied command was not found.
/// - max_levenshtein_distance > Defines the maximum Levenshtein distance between a searched command name
/// and commands. If the distance is lower than or equal to the set distance, it will be display as a
/// suggestion. Setting the distance to 0 will disable suggestions.
/// - indentation_prefix > When you use sub-groups, Serenity will use the `indentation_prefix` to indicate
/// how deeply an item is nested. The default value is "-", however we change it to "~" here.
/// - lacking_permissions > If a user lacks the permissions required to invoke it, we hide the command.
/// - lacking_role > If the user is nothing but lacking a certain role, we simply display it, hence our variant is "Nothing".
/// - wrong_channel > The last enum variant, Strike, ~~strikes~~ a command.
///
///
/// Serenity will automatically analyse and generate a hint or tip explaining the possible cases
/// of ~~strikethrough~~ commands, but only if `strikethrough_commands_tip_in_{dm,guild}` aren't specified.
/// If you pass a value, it will be displayed instead.
#[help]
#[individual_command_tip="Hello! こんにちは！Hola! Bonjour! 您好! 안녕하세요~\n\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text="Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix="~"]
#[lacking_permissions="Hide"]
#[lacking_role="Nothing"]
#[wrong_channel="Strike"]
pub async fn my_help(
   ctx: &Context,
   msg: &Message,
   args: Args,
   opts: &'static HelpOptions,
   groups: &[&'static CommandGroup],
   owners: HashSet<UserId>,
) -> CommandResult
{
   let _ = help_commands::with_embeds(ctx, msg, args, opts, groups, owners).await;
   return Ok(());
}
