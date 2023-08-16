use colored::Colorize;
use reqwest;
use std::collections::HashMap;
use serde_json;
use serde_json::Value;
use std::error::Error;

use crate::errors;

pub fn auto(
    prefix: &str,
    debug: bool,
    apiurl: &str,
    path: &str,
    url: &str,
    quality: &str,
    codec: &str,
    ttwatermark: &str,
    audioformat: &str,
    dublang: &str,
    fullaudio: &str,
    mute: &str,
) {
    println!("{prefix} getting stream URL for {}...", url.color("yellow"));

    let mut getstream_body = HashMap::new();
        getstream_body.insert("url", url);
        getstream_body.insert("vCodec", codec);
        getstream_body.insert("vQuality", quality);
        getstream_body.insert("aFormat", audioformat);
        getstream_body.insert("isNoTTWatermark", ttwatermark);
        getstream_body.insert("isTTFullAudio", fullaudio);
        getstream_body.insert("isAudioMuted", mute);
        getstream_body.insert("dubLang", dublang);

    let getstream_url = &format!("https://{apiurl}/api/json");
    if debug {
        println!(" ");
        println!("{prefix} {}", "====[ debug ]====".color("cyan"));
        println!("{prefix} get stream url request url:");
        println!("{prefix} {}", getstream_url.color("blue"));
        println!("{prefix} get stream url request body:");
        println!("{prefix} {}", serde_json::to_string(&getstream_body).unwrap().color("blue"));
        println!("{prefix} {}", "====[ debug ]====".color("cyan"));
        println!(" ");
    }

    getstream(prefix, &getstream_url, getstream_body, path);
}

#[tokio::main]
async fn getstream(prefix: &str, url: &str, body: HashMap<&str, &str>, path: &str) {
    let client = reqwest::Client::new();
    let response = client.post(url)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .json(&body)
        .send()
        .await;
    let formatted_response = response.expect("method not found in `Result<Response, Error>`").text().await.unwrap();
    println!("{}", formatted_response);
    
    let fmtd_res2: Value = serde_json::from_str(&formatted_response).unwrap();

    if fmtd_res2.get("status").unwrap() == "stream" {
        let streamurl = fmtd_res2.get("url").unwrap().to_string();
        
        let streamurl: &str = &streamurl[1..streamurl.len() - 1];

        let idk: std::result::Result<(), Box<dyn Error + Send + Sync>> = downloadfromstream(prefix, &streamurl.to_string(), path).await;
        println!("{:?}", idk);
    } else {
        errors::create_end(format!("failed to get stream url. {}", fmtd_res2.get("text").unwrap()).as_str());
    }
}

use std::io::Cursor;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
async fn downloadfromstream(prefix: &str, url: &str, path: &str) -> Result<()> {
    let response = reqwest::get(url.to_string()).await?;
    let mut file = std::fs::File::create(format!("{path}/silly.mp4"))?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    println!("{} completed download.", prefix);
    Ok(())
}