use crate::download;
use crate::errors;

use reqwest;
use std::error::Error;
use regex::Regex;
use once_cell::sync::Lazy;

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
    println!(
        "{} starting to get playlist content...",
        prefix
    );

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

    println!(
        "{} completed all downloads to {}",
        prefix, path
    );

}

#[tokio::main]
async fn get_feed(url: &str) -> Result<Vec<Vec<&str>>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let content = client.get("https://www.youtube.com/feeds/videos.xml")
        .query(&[("playlist_id", "PLfobC1Ig9oQEapImB-ZzZ7H07myXmDnhC")])
        .send()
        .await?
        .text()
        .await?;

    let videoids = collect_ids(&content);

    Ok(videoids)
}

pub fn collect_ids(content: &str) -> Vec<Vec<&str>> {
    static REGEX_CORRECTIONS: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"/^<yt:videoId>(.*)<\/yt:videoId>$/g").unwrap());

    let vector_match: Vec<Vec<&str>> = REGEX_CORRECTIONS
        .captures_iter(content)
        .map(|c| c.iter().map(|m| m.unwrap().as_str()).collect())
        .collect();

    return vector_match;
}