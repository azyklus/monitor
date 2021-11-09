/// Represents the Matrix client configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatrixConfig
{
   /// The server to connect with.
   server: String,

   /// The authentication token.
   token: String,

   /// The Matrix app's unique identifier.
   identifier: String,
}

impl MatrixConfig
{
   /// The location of the Matrix config.
   pub const PATH: &'static str = "config/matrix.toml";
}

impl Default for MatrixConfig
{
   fn default() -> MatrixConfig
   {
      let ue: i64 = SystemTime::now().duration_since(UNIX_EPOCH)
         .unwrap()
         .as_secs()
         .try_into()
         .unwrap();

      let dt: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(ue, 0), Utc);
      let ulid: Ulid = Ulid::from_datetime(dt);

      return MatrixConfig{
         server: "https://example.com/".to_string(),
         token: "<TOKEN_HERE>".to_string(),
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
pub fn save(conf: MatrixConfig) -> Result<()>
{
   // Assign the location of our Discord config to a Path variable.
   let fp: &Path = Path::new(MatrixConfig::PATH);
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

/// Loads the configuration located at [`MatrixConfig::PATH`] into the supplied
/// instance of [`MatrixConfig`].
///
///
/// # Examples
///
/// ```should_fail
/// use crate::matrix::config::{self, MatrixConfig};
///
/// fn main()
/// {
///    let mut conf: MatrixConfig = MatrixConfig::new();
///    let _ = config::load(conf).unwrap();
/// }
/// ```
///
/// [`DiscordConfig::PATH`]: crate::matrix::config::MatrixConfig::PATH
/// [`DiscordConfig`]: crate::matrix::config::MatrixConfig
pub fn load(mut conf: MatrixConfig) -> Result<MatrixConfig>
{
   // Assign the Discord config path to a variable.
   let fp: &Path = Path::new(MatrixConfig::PATH);
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
         Err(e) => Err(e.into()),
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


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::errors::*;

use anyhow::Result;

use chrono::{
   DateTime,
   NaiveDateTime,
   Utc,
};

use std::{
   fs::{self, File},
   io::prelude::*,
   path::Path,
   time::{SystemTime, UNIX_EPOCH}
};

use toml::{de, ser};

use ulid::Ulid;
