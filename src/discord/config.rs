use anyhow::Result;

use std::{
   fs::{self, File},
   io::prelude::*,
   path::Path,
};

use toml::{
   ser,
   de,
};

/// Contains configuration values used to interface with Discord.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordConfig
{
   /// The token for accessing the Discord API.
   token: String,

   /// The threshold for caching offline guild members.
   large_threshold: u32,

   /// The number of messages to cache.
   message_cache_size: u32,
}

impl DiscordConfig
{
   /// The location of the config file, relative to the application root.
   pub const PATH: &'static str = "discord.toml";

   /// Returns the API token for the Discord bot.
   #[inline]
   pub fn token(&self) -> String{ return self.token.clone(); }

   /// Returns the specified message cache size for the Discord bot.
   #[inline]
   pub fn cache_size(&self) -> u32{ return self.message_cache_size; }

   /// Returns the large threshold specified in the configuration instance.
   #[inline]
   pub fn large_threshold(&self) -> u32{ return self.large_threshold; }
}

impl Default for DiscordConfig
{
   /// Creates a default instance of the Discord configuration.
   #[inline]
   fn default() -> DiscordConfig
   {
      return DiscordConfig{
         token: "<API_TOKEN>".to_string(),
         large_threshold: 20,
         message_cache_size: 2000,
      };
   }
}

/// Saves the specified configuration to a YAML file located at [`DiscordConfig::PATH`].
///
///
/// # Examples
///
/// ```
/// use crate::discord::config::{self, DiscordConfig};
///
/// fn main()
/// {
///    let conf: DiscordConfig = DiscordConfig::with_values(...);
///    config::save(&conf).unwrap();
/// }
/// ```
///
/// [`DiscordConfig::PATH`]: crate::discord::config::DiscordConfig::PATH
pub fn save(conf: &DiscordConfig) -> Result<()>
{
   let fp: &Path = Path::new(DiscordConfig::PATH);
   let toml: String = ser::to_string(conf).unwrap();

   // Check whether the file we need to write to exists.
   if let Err(e) = fs::metadata(fp) {
      // The file does not exist; return an error.
      return Err(e.into());
   }

   let mut fi: File = File::create(fp)
      .expect("couldn't create the file");

   if let Err(e) = fi.write_all(toml.as_bytes()) {
      return Err(e.into());
   }

   return Ok(());
}

/// Loads the configuration located at [`DiscordConfig::PATH`] into the supplied
/// instance of [`DiscordConfig`].
///
///
/// # Examples
///
/// ```
/// use crate::discord::config::{self, DiscordConfig};
///
/// fn main()
/// {
///    let mut conf: DiscordConfig = DiscordConfig::new();
///    let _ = config::load(&mut DiscordConfig).unwrap();
/// }
/// ```
///
/// [`DiscordConfig::PATH`]: crate::discord::config::DiscordConfig::PATH
/// [`DiscordConfig`]: crate::discord::config::DiscordConfig
pub fn load(conf: &mut DiscordConfig) -> Result<()>
{
   return Ok(());
}
