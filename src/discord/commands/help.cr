require "../app"
require "../command"

# Sends a help message to the user that issue the command.
class Help < Command
   RESPONSE = <<-END
   ===============================================
   === THE MONITOR ===============================
   === ===========================================
   === UTILITY:    ===============================
   === mntr help  > Sends this message. ==========
   === mntr greet > Greets the specified user. ===
   === ===========================================
   === GAMES (WIP): ==============================
   === mntr poker > Starts a poker game. =========
   ===============================================
   END

   def initialize
      @client = Monitor::DISCORD
   end

   def execute
      
   end
end
