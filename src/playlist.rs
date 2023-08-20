use crate::download;
use crate::errors;

use once_cell::sync::Lazy;
use regex::Regex;
use reqwest;
use std::error::Error;
use std::fmt::format;

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

    let mut playlisturl = "nourl";

    if url.starts_with("https://www.youtube.com/playlist?list=") {
        playlisturl = url.strip_prefix("https://www.youtube.com/playlist?list=").unwrap();
    } else if url.starts_with("https://youtube.com/playlist?list=") {
        playlisturl = url.strip_prefix("https://youtube.com/playlist?list=").unwrap();
    } else {
        errors::create_end("invalid playlist url.");
    }

    // @patter@mcr.wtf left a comment on one of my posts (https://wetdry.world/deck/@patter@mcr.wtf/110908024192301245)
    // and told me about the RSS feeds which are used for this and that SIGNIFICANTLY helped with this feature

    let rss_url = format!(
        "https://www.youtube.com/feeds/videos.xml?playlist_id={}",
        playlisturl
    );
    let rss_content = get_feed(&rss_url).unwrap();

    // get ids

    let vididregex = Regex::new(r"(?mU)<yt:videoId>(.*)<\/yt:videoId>").unwrap();
    let result = vididregex.captures_iter(&rss_content);

    // now DOWNLAOD

    println!(
        "{} starting to download videos in playlist... you might be here for a while!",
        prefix
    );

    for mat in result {
        let matchurl = format!("https://youtu.be/{}", mat.get(1).unwrap().as_str());
        if mode == "auto" {
            download::auto(prefix, debug, apiurl, path, &matchurl, quality, codec, ttwatermark, audioformat, dublang, fullaudio, mute);
        } else if mode == "audio" {
            download::auto(prefix, debug, apiurl, path, &matchurl, quality, codec, ttwatermark, audioformat, dublang, fullaudio, mute);
        } else {
            errors::create_end("invalid mode. options: auto, audio");
        }
    }

    println!("{} completed all downloads to {}", prefix, path);
}

fn get_feed(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}