require "./discord/app"

# This module acts as a sort of "main" function.
#
# NOTE: Don't judge, I come from a C/Rust background, so this helps
# the language make sense to me.
module Monitor
   # VERSION denotes the program's version.
   VERSION = "0.1.0"

   Monitor::DISCORD.start
end
