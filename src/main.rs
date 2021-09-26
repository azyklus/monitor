use std::{
   io,
};

#[tokio::main]
async fn main() -> io::Result<()>
{
   let app: discord::Bot = discord::setup().await?;
   app.run().await?;

   Ok(())
}

/// Contains the main logic relating to the Discord bot.
pub mod discord;

extern crate tokio;
