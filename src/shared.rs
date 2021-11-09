lazy_static! {
   pub static ref GLOBAL_CONFIG: AppConfig = AppConfig::default().load().unwrap();
}

/// Application-wide configuration.
///
///
/// # Examples
///
/// Below are some examples of using the global configuration:
///
/// - Get Discord API token from config:
/// ```
/// let config: AppConfig = AppConfig::default();
/// let token = config.discord.token();
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig
{
   /// The *APPLICATION'S* unique identifier.
   identifier: String,

   /// Discord-specific configuration details.
   pub discord: DiscordConfig,

   /// Matrix-specific configuration details.
   pub matrix: MatrixConfig,
}

impl AppConfig
{
   /// The path to the application's configuration.
   pub const PATH: &'static str = "config/app.toml";

   /// Creates a new instance of the `AppConfig`.
   pub fn new(
      identifier: String,
      discord: DiscordConfig,
      matrix: MatrixConfig
   ) -> AppConfig
   {
      return AppConfig{
         identifier,
         discord,
         matrix,
      };
   }

   #[inline]
   pub fn id(&self) -> String { return self.identifier.clone(); }

   /// Saves the app configuration to a file located at [`AppConfig::PATH`].
   ///
   ///
   /// # Examples
   ///
   /// ```
   /// use crate::shared::AppConfig;
   ///
   /// let config = AppConfig::default();
   /// config.save()?;
   /// ```
   ///
   ///
   /// [`AppConfig::PATH`]: crate::shared::AppConfig::PATH
   pub fn save(&self) -> Result<()>
   {
      // Assign our `PATH` constant to a `std::path::Path` variable.
      let fp: &Path = Path::new(AppConfig::PATH);
      // Serialize our config into a String of TOML data.
      let tm: String = toml::to_string(&self).unwrap();

      // Check if our file exists
      if let Ok(true) = fs::try_exists(&fp) {
         // It does, so we'll only try to write our data.

         // Open the file first.
         let mut fi: File = File::open(&fp).expect("couldn't open the file");
         if let Err(e) = fi.write_all(tm.as_bytes()) {
            // The operation failed.
            // Log the failure and return the error.
            log::error!("could not write to configuration file.");
            return Err(e.into());
         } else {
            return Ok(());
         }
      } else {
         // Our config file does not exist, so we must create it.

         let mut fi: File = File::create(&fp).expect("couldn't create the file");
         if let Err(e) = fi.write_all(tm.as_bytes()) {
            // The write operation failed.
            // Log the failure and return the error.
            log::error!("could not write to configuration file.");
            return Err(e.into());
         } else {
            return Ok(());
         }
      }
   }

   /// Loads the app configuration from a file located at [`AppConfig::PATH`].
   ///
   ///
   /// # Examples
   ///
   /// ```
   /// use crate::shared::AppConfig;
   ///
   /// let mut config = AppConfig::default();
   /// config = config.load();
   /// ```
   ///
   ///
   /// [`AppConfig::PATH`]: crate::shared::AppConfig::PATH
   pub fn load(mut self) -> Result<AppConfig>
   {
      // Assign our `PATH` constant to an `std::path::Path` variable.
      let fp: &Path = Path::new(AppConfig::PATH);
      // Create a mutable String variable to hold our data.
      let mut tm: String = String::new();

      // Check whether our config file exists.
      if let Ok(false) = fs::try_exists(&fp) {
         // It does not exist.
         // We must report the error, create the file, and finally return the error.
         log::error!("The app config file does not exist!");
         log::info!("Creating the app config now...");

         let _ = self.save()?;

         return Err(FileError::Nonexistent.into());
      } else {
         // The file exists, so we attempt to open it and return an error if
         // we encounter a problem.

         let mut fi: Result<File, GenericError> = match File::open(&fp) {
            Ok(f) => Ok(f),
            Err(e) => Err(e.into()),
         };

         match fi.unwrap().read_to_string(&mut tm) {
            Ok(_) => {},
            Err(e) => return Err(e.into()),
         }
      }

      self = de::from_str(tm.as_str()).unwrap();

      return Ok(self);
   }
}

impl Default for AppConfig
{
   fn default() -> AppConfig
   {
      let ue: i64 = SystemTime::now().duration_since(UNIX_EPOCH)
         .unwrap()
         .as_secs()
         .try_into()
         .unwrap();

      let dt: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(ue, 0), Utc);
      let ulid: Ulid = Ulid::from_datetime(dt);

      let discord: DiscordConfig = DiscordConfig::default();
      let matrix: MatrixConfig = MatrixConfig::default();

      return AppConfig{
         identifier: ulid.to_string(),
         discord,
         matrix,
      };
   }
}


// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
   discord::config::DiscordConfig,
   matrix::config::MatrixConfig,
};

use automan::errors::*;

use anyhow::Result;

use chrono::{
   DateTime,
   NaiveDateTime,
   TimeZone,
   Utc,
};

use std::{
   fs::{self, File},
   io::prelude::*,
   path::Path,
   time::{SystemTime, UNIX_EPOCH},
};

use toml::de;

use ulid::Ulid;
