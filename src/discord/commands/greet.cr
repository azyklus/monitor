require "../app"
require "../command"

module Commands
   extend self

   class Greet < Command
      def initialize
         @app = Monitor::APP
         @client = self.app.client
         @client_cache = self.client.cache
         @guild = self.client.guild
      end

      def execute
         client.create_message()
      end
   end
end