#[group]
pub struct Games;


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::ShardManagerContainer;

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

