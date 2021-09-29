use serenity::{
   async_trait,
   client::EventHandler,
   model::gateway::Ready,
   prelude::Context,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler
{
   async fn ready(&self, _: Context, ready: Ready)
   {

   }
}

/// General-use commands.
pub mod general;
pub use general::GENERAL_GROUP;
