use crate::download;
use crate::errors;

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

    let mut playlisturl = "nourl";

    if url.starts_with("https://www.youtube.com/playlist?list=") {
        playlisturl = url.strip_prefix("https://www.youtube.com/playlist?list=").unwrap();
    } else if url.starts_with("https://youtube.com/playlist?list=") {
        playlisturl = url.strip_prefix("https://youtube.com/playlist?list=").unwrap();
    } else {
        errors::create_end("invalid playlist url.");
    }

    let rss_url = format!(
        "https://www.youtube.com/playlist?list={}",
        playlisturl
    );

    println!(
        "{} starting to download videos in playlist... you might be here for a while!",
        prefix
    );



    println!("{} completed all downloads to {}", prefix, path);
}

fn scrape_playlist(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}

fn get_feed(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}