initSidebarItems({"enum":[["ClientError","An error returned from the `Client`."]],"fn":[["parse_token","Verifies that the token adheres to the Discord token format and extracts the bot user ID and the token generation timestamp"],["validate_token","Validates that a token is likely in a valid format."]],"mod":[["bridge","A collection of bridged support between the `client` module and other modules."]],"struct":[["Client","The Client is the way to be able to start sending authenticated requests over the REST API, as well as initializing a WebSocket connection through `Shard`s. Refer to the documentation on using sharding for more information."],["ClientBuilder","A builder implementing [`Future`] building a [`Client`] to interact with Discord."],["Context","The context is a general utility struct provided on event dispatches, which helps with dealing with the current “context” of the event dispatch. The context also acts as a general high-level interface over the associated `Shard` which received the event, or the low-level `http` module."],["Extras","A builder to extra things for altering the `Client`."],["TokenComponents","Part of the data contained within a Discord bot token. Returned by [`parse_token`]."]],"trait":[["EventHandler","The core trait for handling events by serenity."],["RawEventHandler","This core trait for handling raw events"]]});