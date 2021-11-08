module Shared
   export
      App,
      DiscordConfig

   include("discord/app.jl")
   include("discord/config.jl")
end
