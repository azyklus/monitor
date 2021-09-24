require "./config"

# Contains the main loop for the Discord bot.
module Monitor
   extend self

   class Bot
      property cache
      property client

      property config : Monitor::DiscordConfig = Monitor::DiscordConfig.new

      def initialize
         @config = self.config.load
         @client = Discord::Client.new(token: self.config.token, client_id: self.config.id)
         @cache  = Discord::Cache.new(client)

         client.cache = self.cache
      end
   end

   # DISCORD is a globally accessible instance of our Discord bot.
   DISCORD = Self::Bot.new

   # Creates a new instance of the Discord bot and sets up the necessary functions.
   # 
   # TODO: Implement function to set up Discord access.
   def discord_setup
   end

   # Gathers and properly initializes all of our commands.
   #
   # TODO: Finish implementing command setup.
   def setup_commands
      prefix : String = "/"
   end
end
