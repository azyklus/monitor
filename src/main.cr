require "./discord/config"

# This module acts as a sort of "main" function.
module Monitor
   # APP_VERSION denotes the program's version.
   APP_VERSION       = "0.1.0"

   # DISCORD_CONFIG is an instance of the applications configuration
   # as pertains to the Discord API. (API token, bot secret, Discord-specific behavior, et cetera).
   DISCORD_CONFIG    = Monitor::DiscordConfig.new.load


end
