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
   pub fn new(identifier: String, discord: DiscordConfig, matrix: MatrixConfig) -> AppConfig
   {
      return AppConfig { identifier, discord, matrix };
   }

   /// Returns the app's ULID.
   #[inline]
   pub fn id(&self) -> String
   {
      return self.identifier.clone();
   }

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
         let mut fi: File = OpenOptions::new().write(true).open(&fp).unwrap();

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
      // Assign our `PATH` constants to `std::path::Path` variables.
      let fp_a: &Path = Path::new(AppConfig::PATH);
      let fp_d: &Path = Path::new(DiscordConfig::PATH);
      let fp_m: &Path = Path::new(MatrixConfig::PATH);

      // Create a set of mutable String variables to hold our data.
      let mut tm_a: String = String::new();
      let mut tm_d: String = String::new();
      let mut tm_m: String = String::new();

      // Check whether our Discord config exists.
      if let Ok(false) = fs::try_exists(&fp_d) {
         // Our Discord config does not exist, so we must create it.
         log::error!("The discord config file does not exist!");
         log::info!("Creating the discord config now...");

         let _ = discord::config::save(self.discord.clone()).unwrap();
      } else {
         // Our Discord config exists, so we only have to load it into our AppConfig instance.
         log::info!("Loading the Discord config...");
         self.discord = discord::config::load(self.discord).unwrap();
      }

      // Check whether our Matrix config exists.
      if let Ok(false) = fs::try_exists(&fp_m) {
         // Our Matrix config does not exist, so we must create it.
         log::error!("The matrix config file does not exist!");
         log::info!("Creating the matrix config now...");

         let _ = matrix::config::save(self.matrix.clone()).unwrap();
      } else {
         // Our Matrix config exists, so we only have to load it into our AppConfig instance.
         log::info!("Loading the Matrix config...");
         self.matrix = matrix::config::load(self.matrix).unwrap();
      }

      // Check whether our App config file exists.
      if let Ok(false) = fs::try_exists(&fp_a) {
         // It does not exist.
         // We must report the error, create the file, and finally return the error.
         log::error!("The app config file does not exist!");
         log::info!("Creating the app config now...");

         let _ = self.save()?;

         return Err(FileError::Nonexistent.into());
      } else {
         // The files exist, so we attempt to open them and return an error if
         // we encounter a problem.

         let mut fi_a: Result<File, GenericError> = match File::open(&fp_a) {
            Ok(f) => Ok(f),
            Err(e) => Err(e.into()),
         };

         match fi_a.unwrap().read_to_string(&mut tm_a) {
            Ok(_) => {}
            Err(e) => return Err(e.into()),
         }

         self = de::from_str(tm_a.as_str()).unwrap();
      }

      return Ok(self);
   }
}

impl Default for AppConfig
{
   fn default() -> AppConfig
   {
      let ue: i64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().try_into().unwrap();

      let dt: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(ue, 0), Utc);
      let ulid: Ulid = Ulid::from_datetime(dt);

      let discord: DiscordConfig = DiscordConfig::default();
      let matrix: MatrixConfig = MatrixConfig::default();

      return AppConfig {
         identifier: ulid.to_string(),
         discord,
         matrix,
      };
   }
}

/// Combines the `save` and `load` methods in [`AppConfig`] into
/// a single helper function.
///
///
/// # Examples
///
/// ```
/// use crate::shared::{load_config, AppConfig};
///
/// let mut config = load_config(AppConfig::default).unwrap();
/// ```
///
///
/// [`AppConfig::PATH`]: crate::shared::AppConfig::PATH
pub fn load_config(mut conf: AppConfig) -> Result<AppConfig>
{
   let mut matrix: MatrixConfig = conf.matrix.clone();
   let mut discord: DiscordConfig = conf.discord.clone();
   let mut app: AppConfig = conf.clone();

   if let Ok(true) = fs::try_exists("config/discord.toml") {
      let mut tml: String = String::new();

      let mut fi = File::open("config/discord.toml").unwrap();
      fi.read_to_string(&mut tml).unwrap();

      discord = toml::from_str(tml.as_str()).unwrap();
   } else {
      log::error!("Discord config file does not exist.");
      log::error!("Creating it now...");

      let mut tml = toml::to_string_pretty(&discord).unwrap();

      let mut fi = File::create("config/discord.toml").unwrap();
      fi.write_all(tml.as_bytes()).unwrap();
   }

   if let Ok(true) = fs::try_exists("config/matrix.toml") {
      let mut tml: String = String::new();

      let mut fi = File::open("config/matrix.toml").unwrap();
      fi.read_to_string(&mut tml).unwrap();

      matrix = toml::from_str(tml.as_str()).unwrap();
   } else {
      log::error!("Matrix config file does not exist.");
      log::error!("Creating it now...");

      let mut tml = toml::to_string_pretty(&matrix).unwrap();

      let mut fi = File::create("config/matrix.toml").unwrap();
      fi.write_all(tml.as_bytes()).unwrap();
   }

   if let Ok(true) = fs::try_exists("config/app.toml") {
      let mut tml: String = String::new();

      let mut fi = File::open("config/app.toml").unwrap();
      fi.read_to_string(&mut tml).unwrap();

      app = toml::from_str(tml.as_str()).unwrap();
      app.discord = discord;
      app.matrix = matrix;

      conf = app;

      conf.save().unwrap();
   } else {
      log::error!("App config file does not exist.");
      log::error!("Creating it now...");

      app.discord = discord;
      app.matrix = matrix;

      let mut tml: String = toml::to_string_pretty(&app).unwrap();

      let mut fi = File::create("config/app.toml").unwrap();
      fi.write_all(tml.as_bytes()).unwrap();

      log::info!("Done!");

      conf = app;
   }

   return Ok(conf);
}

// IMPORTS //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
   discord::{self, config::DiscordConfig},
   matrix::{self, config::MatrixConfig},
   errors::*,
};

use anyhow::Result;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use std::{
   fs::{self, File, OpenOptions},
   io::prelude::*,
   path::Path,
   time::{SystemTime, UNIX_EPOCH},
};

use toml::de;

use ulid::Ulid;
