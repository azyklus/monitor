import Discord

mutable struct DiscordApp
   client::Discord.Client
   discord_config::DiscordConfig
   app_config::AppConfig
end

"""
   init_app()

Initialises the main application instance.
"""
function init_app()::App
   return App(client, config)
end

"""
   load_config()

Loads the app configuration from a file.
"""
function load_config()::AppConfig
end

"""
   save_config()

Saves the app configuration to a file.
"""
function save_config()
end
