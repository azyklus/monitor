module Automan
   export
      start,
      Ulid,
      Util,
      NAME,
      VERSION

   const NAME    = "Automan"
   const VERSION = "v0.1.0"

   include("Shared.jl")
   include("Ulid.jl")
   include("Util.jl")

   function start()
      println(Ulid.ulid())
   end
end

# Start the application.
Automan.start()
