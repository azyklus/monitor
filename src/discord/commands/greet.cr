require "../app"
require "../command"

module Commands
   extend self

   # Command: Greet
   #
   # Greets the specified user.
   #
   # ## Usage
   # 
   # `mntr greet <user.mention>`: The bot responds with a message in the channel
   # where it received the message saying "<greeting user> says hi to <greeted user>!"
   class Greet < Command
      def initialize
         @app = Monitor::DISCORD
         @client = self.app.client
         @client_cache = self.client.cache
         @guild = self.client.guild
      end

      # Sends a message to greet the specified user.
      #
      # :inherit:
      def execute
      end
   end
end
