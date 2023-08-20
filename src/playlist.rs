use crate::download;
use crate::errors;

use once_cell::sync::Lazy;
use regex::Regex;
use reqwest;
use std::error::Error;

pub fn download(
    prefix: &str,
    debug: bool,
    mode: &str,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codec: &str,
    ttwatermark: bool,
    audioformat: &str,
    dublang: bool,
    fullaudio: bool,
    mute: bool,
) {
    println!("{} starting to get playlist content...", prefix);

    let playlisturl = "nourl";

    if url.starts_with("https://www.youtube.com/playlist?list=") {
        let playlisturl = url.strip_prefix("https://www.youtube.com/playlist?list=");
    } else if url.starts_with("https://youtube.com/playlist?list=") {
        let playlisturl = url.strip_prefix("https://youtube.com/playlist?list=");
    } else {
        errors::create_end("invalid playlist url.");
    }

    // @patter@mcr.wtf left a comment on one of my posts (https://wetdry.world/deck/@patter@mcr.wtf/110908024192301245)
    // and told me about the RSS feeds which are used for this and that SIGNIFICANTLY helped with this feature

    let videoids = get_feed(playlisturl);

    println!(
        "{} starting to download videos in playlist... you might be here for a while!",
        prefix
    );

    println!("{:?}", videoids);
    if let Ok(video) = videoids {
        println!("{:?}", video);
    }

    println!("{} completed all downloads to {}", prefix, path);
}

#[tokio::main]
async fn get_feed(url: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let content = client
        .get("https://www.youtube.com/feeds/videos.xml")
        .query(&[("playlist_id", "PLfobC1Ig9oQEapImB-ZzZ7H07myXmDnhC")])
        .send()
        .await?
        .text()
        .await?;

    let videoids = collect_ids(&content);

    println!("shit {:?}", videoids);

    Ok(())
}

pub fn collect_ids(content: &str) -> &str {
    let re = Regex::new(r"/^<yt:videoId>(.*)<\/yt:videoId>$/g").unwrap();

    println!("{:?}", re.find(content));

    return "e";
}
