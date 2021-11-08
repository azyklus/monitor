struct DiscordConfig
   token::String
   threshold::UInt32
   cache_size::UInt32
end

const DISCORD_CONFIG_PATH = "config/discord.toml"

"""
   load_config()

Loads the app configuration from a file.
"""
function load_config()::DiscordConfig
end

"""
   save_config()

Saves the app configuration to a file.
"""
function save_config(config::DiscordConfig)
end

