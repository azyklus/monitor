require "yaml"

module Monitor
   extend self

   # DiscordConfig is a collection of config values for the Discord API.
   struct DiscordConfig
      include YAML::Serializable

      # The configuration path.
      PATH = "discord.yaml"

      # The Discord API token.
      @[YAML::Field(key: "token")]
      property token : String

      # The integer ID of the client.
      @[YAML::Field(key: "client_id")]
      property id    : UInt64
   
      # The message cache size.
      # 
      # NOTE: the type for this property is `UInt32` because the compiler
      # throws a fit if we set it specifically to `Int` for some reason.
      @[YAML::Field(key: "message_cache_size")]
      property cache_size : UInt32

      # Represents the number of users in a large guild before no longer
      # caching offline members.
      #
      # NOTE: the type for this property is `UInt32` because the compiler
      # throws a fit if we set it specifically to `Int` for some reason.
      @[YAML::Field(key: "large_threshold")]
      property large_threshold : UInt32

      def initialize
         @token            = ""
         @id               = 0_u64
         @cache_size       = 2000
         @large_threshold  = 200
      end

      # Saves the current instance to a YAML file.
      #
      # ## Saving your configuration
      #
      # ```crystal
      # config = Discord::Config.new
      # config.save
      def save
         yaml = self.to_yaml

         if File.exists?(PATH)
            if File.empty?(PATH)
               File.write(PATH, yaml)
            else
               raise "config file already exists and is not empty"
            end
         else
            file = File.write(PATH, yaml)
         end
      end

      # Copies the YAML config into the current instance.
      #
      # ## Loading your configuration
      # 
      # ```crystal
      # config = Discord::Config.new
      # config = config.load
      def load : Config
         if File.exists?(PATH)
            if File.empty?(path)
               self.save
               raise "config file was empty"
            end

            yaml = File.open(PATH) do |file|
               YAML.parse(file)
            end

            return Config.from_yaml(yaml)
         else
            raise "config file does not exist"
         end
      end
   end
end
