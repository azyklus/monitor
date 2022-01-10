/// The main Matrix 'bot' structure. Contains the client, configuration, and everything else
/// the program needs to run.
#[non_exhaustive]
pub struct MatrixBot
{
   config: MatrixConfig,
}

impl MatrixBot
{
   /// Creates a new instance of the Matrix bot.
   pub fn new(config: &MatrixConfig) -> Result<MatrixBot, GenericError>
   {
      return Ok(MatrixBot{
         config: config.clone(),
      });
   }

   /// Runs the bot.
   pub async fn run(&mut self) -> Result<()>
   {
      return Ok(());
   }
}


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
   errors::GenericError,
   matrix::config::MatrixConfig,
};

use anyhow::Result;
