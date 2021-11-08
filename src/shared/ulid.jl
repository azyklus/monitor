module Ulid
   import Random, Dates

   export ulid,
         encode_time,
         encode_rand,
         ENCODING,
         BASE

   const ENCODING = "0123456789ABCDEFGHJKMNPQRSTVWXYZ"
   const BASE = 32

   @inline prng() = Random.rand(Random.RandomDevice(), UInt16) / 0xFFFF

   function encode_time(now::Int, n::Int)::String
      # `now` should be in milliseconds-since-Epoch.
      s = Vector{Char}(undef, n)

      @inbounds for i = n:-1:1
         m = now % BASE
         s[i] = ENCODING[m+1]
         now = (now - m) ÷ BASE
      end

      return String(s)
   end

   function encode_rand(n::Int)::String
      s = Vector{Char}(undef, n)

      @inbounds for i = n:-1:1
         r = floor(Int, BASE * prng()) + 1
         s[i] = ENCODING[r]
      end

      return String(s)
   end

   """
      ulid()

   Generate a universally unique lexicographically sortable identifier as a String.
   """
   ulid() = encode_time(trunc(Int, Dates.datetime2unix(Dates.now()) * 1000), 10) * encode_rand(16)

end
