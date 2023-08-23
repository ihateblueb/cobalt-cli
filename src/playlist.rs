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

    //let rss_content = get_feed(&rss_url).unwrap();

    //let vididregex = Regex::new(r"(?mU)<yt:videoId>(.*)<\/yt:videoId>").unwrap();
    //let result = vididregex.captures_iter(&rss_content);

    println!(
        "{} starting to download videos in playlist... you might be here for a while!",
        prefix
    );

    // ytInitialData = [^{]*(.*?); *</script>

        let silly = scrape_playlist(&rss_url).unwrap();

        let presillyregex = Regex::new(r#"ytInitialData = [^{]*(.*?); *</script>"#).unwrap();
        let sillyregex = Regex::new(r#"(?mU)\{\\"videoId\\":\\"(.*)\\","#).unwrap();

        //{\"videoId\":\"
        // \",

        let presilly_r = presillyregex.captures_iter(&silly);

        for mat in presilly_r {
            let mut ytdata = mat.unwrap();
        }

        let silly_r = sillyregex.captures_iter(ytdata);

        for mat in silly_r {

        }

    /*
    
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

    */

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