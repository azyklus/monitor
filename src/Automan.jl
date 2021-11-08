module Automan
   export
   start,
   APP_ID,
   LICENSE,
   NAME,
   VERSION

   include("Shared.jl")

   const NAME    = "Automan"
   const VERSION = "v0.1.0"
   const LICENSE = "BSD 3-Clause License (/license.md)"
   const APP_ID = Shared.Ulid.ulid()

   function start()
      println(NAME)
      println(VERSION)
      println(LICENSE)
      println(IDENTIFIER)
      println("||-------------------||")

      discord_app = Shared.init_discord()
   end
end

import Automan

# Start the application.
Automan.start()
