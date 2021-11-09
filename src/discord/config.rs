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

   /// The bot's unique identifier.
   identifier: String,
}

impl DiscordConfig
{
   /// The location of the config file, relative to the application root.
   pub const PATH: &'static str = "config/discord.toml";

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
      let ue: i64 = SystemTime::now().duration_since(UNIX_EPOCH)
         .unwrap()
         .as_secs()
         .try_into()
         .unwrap();

      let dt: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(ue, 0), Utc);
      let ulid: Ulid = Ulid::from_datetime(dt);

      return DiscordConfig{
         token: "<API_TOKEN>".to_string(),
         large_threshold: 20,
         message_cache_size: 2000,
         identifier: ulid.to_string(),
      };
   }
}

/// Saves the specified configuration to a YAML file located at [`DiscordConfig::PATH`].
///
///
/// # Examples
///
/// ```should_fail
/// use crate::discord::config::{self, DiscordConfig};
///
/// fn main()
/// {
///    let conf: DiscordConfig = DiscordConfig::with_values(...);
///    config::save(conf).unwrap();
/// }
/// ```
///
/// [`DiscordConfig::PATH`]: crate::discord::config::DiscordConfig::PATH
pub fn save(conf: DiscordConfig) -> Result<()>
{
   // Assign the location of our Discord config to a Path variable.
   let fp: &Path = Path::new(DiscordConfig::PATH);
   // Serialize our config into a String.
   let toml: String = ser::to_string(&conf).unwrap();

   // Check whether the file we need to write to exists.
   if let Ok(false) = fs::try_exists(fp) {
      // It does not, so we must create it.

      // Create the file, as it does not exist, erroring if
      // there is a problem doing so.
      let mut fi: File = File::create(fp)
         .expect("couldn't create the file");

      // Write our TOML to the config file.
      if let Err(e) = fi.write_all(toml.as_bytes()) {
         // Return an error if we run into a problem while we're
         // writing to the file.
         return Err(e.into());
      }
   } else {
      return Err(FileError::Exists.into());
   }

   // Return "Ok".
   return Ok(());
}

/// Loads the configuration located at [`DiscordConfig::PATH`] into the supplied
/// instance of [`DiscordConfig`].
///
///
/// # Examples
///
/// ```should_fail
/// use crate::discord::config::{self, DiscordConfig};
///
/// fn main()
/// {
///    let mut conf: DiscordConfig = DiscordConfig::new();
///    let _ = config::load(conf).unwrap();
/// }
/// ```
///
/// [`DiscordConfig::PATH`]: crate::discord::config::DiscordConfig::PATH
/// [`DiscordConfig`]: crate::discord::config::DiscordConfig
pub fn load(mut conf: DiscordConfig) -> Result<DiscordConfig>
{
   // Assign the Discord config path to a variable.
   let fp: &Path = Path::new(DiscordConfig::PATH);
   // Create an empty string to house our TOML.
   let mut toml: String = String::new();

   // Check whether the config file already exists.
   if let Ok(false) = fs::try_exists(fp) {
      log::error!("File does not exist...");
      log::error!("Creating it now.");

      // It does not, so we create it...
      self::save(conf)?;
      // ...and return an error.
      return Err(FileError::Nonexistent.into());
   } else {
      // It exists, so we open the file, erroring if
      // there is an issue.
      let mut fi: Result<File, GenericError> = match File::open(fp) {
         Ok(file) => Ok(file),
         Err(e) => {
            self::save(conf)?;

            Err(e.into())
         },
      };

      // Read from the config file into the TOML variable.
      fi
         .unwrap()
         .read_to_string(&mut toml)
         .expect("error reading file");
   }

   // Deserialize the TOML, erroring if there is
   // an issue.
   conf = de::from_str(toml.as_str())
      .expect("error deserializing toml");

   // Return our config.
   return Ok(conf);
}

use automan::errors::{FileError, GenericError};

use anyhow::Result;

use chrono::{DateTime, NaiveDateTime, Utc};

use std::{
   fs::{self, File},
   io::prelude::*,
   path::Path,
   time::{SystemTime, UNIX_EPOCH},
};

use toml::{
   ser,
   de,
};

use ulid::Ulid;
