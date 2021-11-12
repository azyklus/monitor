# The Monitor

The Monitor is a simple Discord bot that hangs out in the [Narwhals](https://discord.gg/GyXtwnBWne) server.
The bot offers several chat games to play and many tools to automatically perform administrative functions.


## Resources

- [Discord Developers - Documentation](https://discord.com/developers/docs)
- [Discord Developers - Applications Page](https://discord.com/developers/applications)
- [Serenity Library](https://github.com/serenity-rs/serenity)
- [Matrix API Specification](https://matrix.org/docs/spec/)
- [Matrix SDK - Rust](https://github.com/matrix-org/matrix-rust-sdk)


## Creating a new instance

If you would like to create a new instance of the Monitor for whatever reason, you may do so by one of two methods.
You may build from source or you may download one of the Release version from [here.](https://github.com/mnimi/monitor/releases)

### Building from source

*NOTE: please install `cargo-make` to build this project.*

If you wish to run your own instance of Monitor, follow these instructions:
- Clone this repository (`git clone https://github.com/mnimi/monitor.git`).
- Build the bot (`makers build`).
- Run the program (NOTE: You must supply your own token via command arguments or config file).

### Release package

Download the ZIP file in the Releases section of this repository.
It will contain the proper environment to get the bot up and running.

And in either case, you will need to follow these instructions if you don't already know what you're doing:

You will need to create an Application on [Discord's Developers page](https://discord.com/developers/applications).
You can do this by clicking the link above and clicking on 'New Application' in the top right corner next to your user avatar.

Once you have created the application, you will need your bot token from the 'Bot' tab in your app's info page.
Copy this token and then paste it into the `token` field in the `config/discord.toml` file.

Now you can simply double click on the 'Monitor.exe' file in the root folder and a terminal/console window will open, showing you
the application's logs in real time. (These logs will also be saved in `/.logfile`.)


## Development

If you wish to develop extensions for the Monitor, you are able to do so through the `automan` support library present in this repository.
`automan` is designed to be extensible but also to provide certain conveniences such as easy bot setup and logging.
Such things cannot be easily parsed from the project, and should be used when possible.

To create an extension, you will need to make use of the project's `automan::extensions::*` modules and their accompanying helpers.  

*NOTE: this is coming in the `0.2` version of the Monitor.*


## Contributing

1. Fork it (<https://github.com/your-github-user/monitor/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request


## Contributors

- [Yarot Kell](https://twitter.com/yarotk) - creator and maintainer
