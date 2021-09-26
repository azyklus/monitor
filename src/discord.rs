use std::io;
pub use self::app::Bot;

pub async fn setup() -> io::Result<Bot>
{
   Ok(Bot{})
}

/// Contains the main bot loop.
pub mod app;
