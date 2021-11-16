/// The 'Chat' command group.
/// 
/// This contains commands related directly to various chat usages.
#[group]
#[commands(
   delete,
   wipe,
   slowmode,
   gif
)]
pub struct Chat;

/// Deletes the specified number of messages, up to 99.
///
///
/// # Examples
///
/// `mntr rm 10` > Deletes 10 messages, plus the message that contained the command.
/// `mntr del 92` > Deletes 93 messages.
/// `mntr delete 68` > Deletes 69 messages. Nice.
///
///
/// # Notes
///
/// This command cannot be used to delete messages that are greater than two weeks
/// of age. Those will have to be deleted manually.
#[command]
#[aliases("del", "rm")]
pub async fn delete(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
   let n: u64 = args.parse()?;
   let r: RangeInclusive<_> = RangeInclusive::new(2, 100);

   if !r.contains(&n) {
      log::warn!("an invalid integer value was supplied to the 'delete' command");
      let _ = msg.channel_id.send_message(&ctx.http, |m| {
         m.embed(|e| {
            e.title("Error!");

            e.author(|a| {
               a.icon_url(&msg.author.avatar_url().unwrap());
               a.name("The Monitor");

               a
            });

            e.description("There was an invalid integer passed to the `delete` command.");

            e.footer(|f| {
               f.text("Run the help command to see more information about this command.")
            });

            e
         })
      }).await;
   }

   let msgs = msg.channel_id.messages(&ctx.http, |r| {
      r.before(msg.id).limit(n)
   }).await?;

   let mut msg_ids: Vec<MessageId> = vec![];
   for m in msgs.iter() {
      let id = m.id;
      msg_ids.push(id);
   }

   if let Err(e) = msg.channel_id.delete_messages(&ctx.http, &msg_ids).await {
      log::error!("encountered an error whilst trying to delete the messages");
      return Err(e.into());
   }

   if let Err(e) = msg.delete(&ctx.http).await {
      log::error!("encountered an error trying to delete the command message");
      return Err(e.into());
   }

   let mut m: Message = msg.reply(&ctx.http, format!("Successfully deleted {} messages!", msg_ids.len())).await?;
   let mut d: Duration = Duration::seconds(100);
   loop {
      let d2: Duration = Duration::seconds(1);
      if !d.is_zero() {
         d = d.checked_sub(&d2).unwrap();
         continue;
      } else {
         m.delete(&ctx.http).await?;
         break;
      }
   }

   return Ok(());
}

/// Deletes the last hundred messages from the channel.
///
///
/// # Examples
///
/// `mntr wipe` > This will delete 99 messages, plus the message that triggered the
/// command.
///
///
/// # Notes
///
/// This command cannot be used to delete messages that are greater than two weeks
/// of age as Discord's API forbids it. Those messages will have to be manually removed.
#[command]
#[aliases("clean", "sanitize", "rmr")]
pub async fn wipe(ctx: &Context, msg: &Message) -> CommandResult
{
   let msgs = msg.channel_id.messages(&ctx.http, |r| {
      r.before(msg.id).limit(100)
   }).await?;

   let mut msg_ids: Vec<MessageId> = vec![];
   for m in msgs.iter() {
      let id = m.id;
      msg_ids.push(id);
   }

   if let Err(e) = msg.channel_id.delete_messages(&ctx.http, &msg_ids).await {
      log::error!("encountered an error whilst trying to delete the messages");
      return Err(e.into());
   }

   let mut m: Message = msg.reply(&ctx.http, "Successfully deleted 100 messages!").await?;
   let mut d: Duration = Duration::seconds(100);
   loop {
      let d2: Duration = Duration::seconds(1);
      if !d.is_zero() {
         d = d.checked_sub(&d2).unwrap();
         continue;
      } else {
         m.delete(&ctx.http).await?;
         break;
      }
   }

   return Ok(());
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
         }).await;
         return Err(e.into());
      } else {
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
         }).await;
      }
   } else if let Some(Channel::Guild(channel)) = msg.channel_id.to_channel_cached(&ctx.cache).await {
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
      }).await;
   } else {
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
      }).await;
   }

   return Ok(());
}

/// Sends a random meme as a reply to the trigger message.
#[command]
pub async fn gif(ctx: &Context, msg: &Message) -> CommandResult
{
   let mut rnd = rngs::OsRng;
   let num = rnd.gen_range(0..25) as usize;

   let gifs: Vec<Gif> = (&GIPHY).trending().await.unwrap();

   let _ = msg.channel_id.send_message(&ctx.http, |m| {
      m.content(&gifs[num].url)
   }).await;

   return Ok(());
}

// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
   GIPHY,
};

use automan::{
   ShardManagerContainer,
   gif::GiphyBot,
};

use chrono::Duration;

use giphy::v1::Gif;

use rand::{rngs, Rng, RngCore};

use std::{
   collections::HashSet,
   ops::RangeInclusive,
};

use serenity::{
   client::{
      Context,
      bridge::gateway::ShardId,
   },
   framework::standard::{
      Args,
      CommandGroup,
      CommandResult,
      help_commands,
      macros::{
         command,
         group,
      },
   },
   model::{
      channel::{Channel, Message},
      prelude::{MessageId, UserId},
   },
   prelude::*,
};
