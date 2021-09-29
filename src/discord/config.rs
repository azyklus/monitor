

/// Contains configuration values used to interface with Discord.
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
   pub const PATH: &'static str = "discord.yml";

   /// Returns the API token for the Discord bot.
   #[inline]
   pub fn token(&self) -> String{ return self.token; }

   /// Returns the specified message cache size for the Discord bot.
   #[inline]
   pub fn cache_size(&self) -> u32{ return self.message_cache_size; }

   /// Returns the large threshold specified in the configuration instance.
   #[inline]
   pub fn large_threshold(&self) -> u32{ return self.large_threshold; }
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
pub fn save(conf: &DiscordConfig) -> anyhow::Result<()>
{

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
pub fn load(conf: &mut DiscordConfig) -> anyhow::Result<()>
{}
