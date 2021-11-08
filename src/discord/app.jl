import Discord

mutable struct DiscordApp
   client::Discord.Client
   config::DiscordConfig

   DiscordApp(client::Discord.Client, config::DiscordConfig) = new(client, config)
end

"""
   init_app()

Initialises the main application instance.
"""
function init_discord()::DiscordApp
   config::DiscordConfig = DiscordConfig("", 20, 2000)

   if isfile(DISCORD_CONFIG_PATH)
      config = load_config()
   else
      save_config(config)
   end

   return DiscordApp(client, discord_config, app_config)
end
