use serenity::{
   async_trait,
   client::EventHandler,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler{}

/// General-use commands.
pub mod general;
pub use general::GENERAL_GROUP;
