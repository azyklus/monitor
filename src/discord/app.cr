require "discordcr"
require "./config"

# Contains the main loop for the Discord bot.
module Monitor
   extend self

   # The Discord bot object.
   class Bot
      property cache
      property client
      property config

      PREFIX = "mntr "

      def initialize
         @config = DiscordConfig.new
         @client = Discord::Client.new(token: self.config.token, client_id: self.config.id)
         @cache  = Discord::Cache.new(self.client)

         @client.cache = self.cache

         @client.on_message_create do |payload|
            msg = payload.content
            
            case msg
            when PREFIX + "help"
               help = Commands::Help.new
               help.execute
            else
               @client.create_message(payload.channel_id, "```\nPlease issue a valid command!\n```")
            end
         end
      end

      # Start the Discord bot.
      def start
         self.client.run
      end
   end

   # DISCORD is a globally accessible instance of our Discord bot.
   DISCORD = Bot.new


   # Creates a new instance of the Discord bot and sets up the necessary functions.
   # 
   # TODO: Implement function to set up Discord access.
   def setup_discord
   end
end
