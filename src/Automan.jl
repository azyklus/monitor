module Automan
   export
      start,
      app_id,
      NAME,
      VERSION

   const NAME    = "Automan"
   const VERSION = "v0.1.0"

   include("Shared.jl")

   function app_id()::String
      return Shared.Ulid.ulid()
   end

   function start()
      println(app_id())
   end
end

import Automan

# Start the application.
Automan.start()
