module Automan
   export
   start,
   APP_ID,
   NAME,
   VERSION


   include("Shared.jl")

   const NAME    = "Automan"
   const VERSION = "v0.1.0"
   const APP_ID = Shared.Ulid.ulid()

   function start()
      println(APP_ID)
   end
end

import Automan

# Start the application.
Automan.start()
