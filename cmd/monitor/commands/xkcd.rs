/// The Xkcd commands fetch comics straight from Xkcd itself.
///
/// - `xkcd random` will get a random comic.
/// - `xkcd latest` will get the latest comic.
/// - `xkcd select` will get the specified comic. (This command takes an integer argument.)
///
/// 
/// # Notes
/// 
/// A `search` command has not yet been implemented but may be at some point in the future.
/// Right now, the three base commands are the most important for the Xkcd group.
#[group]
#[commands(latest, random, select)]
pub struct Xkcd;

/// Represents an Xkcd comic.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct XkcdComic
{
    pub month: String,
    pub num: usize,
    pub link: String,
    pub year: String,
    pub news: String,
    pub safe_title: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    pub day: String,
}

/// Gets a random comic from Xkcd.
#[command]
pub async fn random(ctx: &Context, msg: &Message) -> CommandResult
{
    let mut rnd = rngs::OsRng;
    let mut num: usize = {
        // Send a request to XKCD's JSON web API to get the latest comic.
        let bod1 = reqwest::get("https://xkcd.com/info.0.json")
            .await
            .expect("failed to retrieve the comic from XKCD")
            // Separate the body from the rest of the HTTP request.
            .text()
            .await
            .expect("failed to separate the body of the request");
    
        // Deserialize our received JSON into our comic structure.
        let comic1: XkcdComic = de::from_str(bod1.as_str()).exepct("failed to deserialize JSON data");

        // Generate our number.
        rnd.gen_range(1..=comic1.num) as usize
    };

    // Send a request to XKCD's JSON web API to get a random comic,
    // using the random number generated above.
    let url2 = format!("https://xkcd.com/{}/info.0.json", num);
    let bod2 = reqwest::get(url)
        .await
        .expect("failed to retrieve the comic from XKCD")
        // Separate the body from the rest of the HTTP request.
        .text()
        .await
        .expect("failed to separate the body from the request");

    // Deserialize our received JSON data into our comic structure.
    let comic2: XkcdComic = de::from_str(bod2.as_str()).expect("failed to deserialize JSON data");

    // Request the image from XKCD.
    let image_res = reqwest::get(comic2.img)
        .await
        .expect("failed to retrieve image from XKCD");

    // Create the file.
    let mut dst: File = {
        let fname = image_res
            // Get the URL.
            .url()
            // Get path segments.
            .path_segments()
            // Get the final segment.
            .and_then(|seg| seg.last())
            // Get the name of the file.
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("temp.image");
        
        File::create(fname).expect("failed to create file")
    };

    // Get the body of the request.
    let content = image_res.text().await.expect("failed to separate body from the request");
    // Save the image.
    io::copy(&mut content.as_bytes(), &mut dst).expect("failed to save image");

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.add_file(dst);
    }).await?;

    return Ok(());
}

/// Gets the latest comic posted to Xkcd.
#[command]
pub async fn latest(ctx: &Context, msg: &Message) -> CommandResult
{
    let bod = reqwest::get("https://xkcd.com/info.0.json")
        .await
        .expect("failed to retrieve the comic from XKCD")
        .text()
        .await
        .expect("failed to separate the body of the request");
    
    // Deserialize our received JSON into our comic structure.
    let comic: XkcdComic = de::from_str(bod.as_str()).expect("failed to deserialize JSON data");

    // Request the image from XKCD.
    let image_res = reqwest::get(comic.img)
        .await
        .expect("failed to retrieve image from XKCD");

    // Create the file.
    let mut dst: File = {
        let fname = image_res
            // Get the URL.
            .url()
            // Get path segments.
            .path_segments()
            // Get the final segment.
            .and_then(|seg| seg.last())
            // Get the name of the file.
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("temp.image");
        
        File::create(fname).expect("failed to create file")
    };

    // Get the body of the request.
    let content = image_res.text().await.expect("failed to separate body from the request");
    // Save the image.
    io::copy(&mut content.as_bytes(), &mut dst).expect("failed to save image");

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.add_file(dst);
    }).await?;

    return Ok(());
}

/// Gets a specific comic from XKCD.
///
/// 
/// # Usage
///
/// - `mntr xkcd select 4` gets `https://xkcd.com/4`
#[command]
pub async fn select(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
    let param = args.single::<usize>().expect("expected a single integer");
    
    // Request the comic from XKCD.
    let url = format!("https://xkcd.com/{}/info.0.json", param);
    let bod = reqwest::get(url)
        .await
        .expect("failed to retrieve comic from XKCD")
        // Separate the body from the rest of the HTTP request.
        .text()
        .await
        .expect("failed to separate body from the request");

    // Deserialize our received JSON into our comic structure.
    let comic: XkcdComic = de::from_str(bod.as_str()).expect("failed to serialize JSON data");

    // Request the image.
    let image_res = reqwest::get(comic.img).await.expect("failed to retrieve image from XKCD");

    // Create the file.
    let mut dst: File = {
        let fname = image_res
            // Get the URL.
            .url()
            // Get path segments.
            .path_segments()
            // Get the final segment.
            .and_then(|seg| seg.last())
            // Get the name of the file.
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("temp.image");
            
        File::create(fname).expect("failed to create file")
    };
    
    // Get the body of the request.
    let content = image_res.text().await.expect("failed to separate body from the request");
    // Save the image.
    io::copy(&mut content.as_bytes(), &mut dst).expect("failed to save image");
    
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.add_file(dst);
    }).await?;
    
    return Ok(());
}


// IMPORTS ////////////////////////////////////////////////////////////////////////////////////////

use automan::{
    ShardManagerContainer,
};

use rand::{Rng, RngCore, rngs};

use std::{
    collections::HashSet,
    io::{self, File},
    ops::RangeInclusive,
};

use serde_json::de;

use serenity::{
    client::{
       Context,
       bridge::gateway::ShardId,
    },
    framework::standard::{
       Args,
       CommandGroup,
       CommandResult,
       help_commands,
       macros::{
          command,
          group,
       },
    },
    model::{
       channel::{Channel, Message},
       prelude::{MessageId, UserId},
    },
    prelude::*,
};
