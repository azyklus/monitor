module Shared
   export
   DiscordApp,
   MatrixApp,
   AppConfig,
   DiscordConfig,
   MatrixConfig

   include("discord/config.jl")
   include("matrix/config.jl")

   include("shared/config.jl")
   include("shared/ulid.jl")
   include("shared/util.jl")

   include("discord/app.jl")
   include("matrix/app.jl")
end
