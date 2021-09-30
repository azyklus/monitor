use serenity::{
   async_trait,
   client::EventHandler,
   model::gateway::Ready,
   prelude::*,
};

use std::collections::{HashMap, HashSet};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler
{
   async fn ready(&self, _: Context, ready: Ready)
   {
      trace!("{} is connected!", ready.user.name);
   }
}

#[doc(hidden)]
pub struct CommandCounter;

impl TypeMapKey for CommandCounter
{
   type Value = HashMap<String, u64>;
}

/// General-use commands.
pub mod general;
pub use general::MY_HELP;
pub use general::GENERAL_GROUP;
