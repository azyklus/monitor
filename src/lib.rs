#![crate_name="automan"]
#![crate_type="lib"]
#![deny(clippy::all)]
#![warn(missing_docs)]
#![allow(unused)]
#![allow(clippy::needless_return)]
#![allow(dead_code)]
#![feature(path_try_exists)]

//! The Monitor is a simple Discord bot that hangs out in the [Narwhals] server.
//! The bot offers several chat games as well as tools to automatically perform certain
//! administrative tasks in order to better keep the server organized.
//!
//! You can add the bot to your own server [here], or you can build your own by following
//! [these instructions].
//!
//! [Narwhals]: https://discord.gg/GyXtwnBWne
//! [here]: https://discord.com/api/oauth2/authorize?client_id=817894435299262516&permissions=8&redirect_uri=https%3A%2F%2Fdiscord.com%2Fapi%2Foauth2%2Fauthorize%3Fclient_id%3D817894435299262516%26scope%3Dapplications.commands&scope=bot
//! [these instructions]: https://github.com/mnimi/monitor/#installation

/// A supplimentary implementation of the Matrix API.
///
/// Something to NOTE: This module is a work-in-progress and
/// will eventually supplant the "official" Matrix API crate
/// in our project.
pub mod __matrix;

/// Contains error types.
pub mod errors;

/// Contains light implementations of various chat games.
pub mod games;
