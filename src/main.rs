use clap::{Arg, Command};
use colored::Colorize;
use std::env;

mod download;
mod errors;
mod playlist;

static DEBUG: bool = false;
static PREFIX: &'static str = "[cobalt]";

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        // mode
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .help("set which mode to download with (default: auto, other: audio)"),
        )
        // universal
        .arg(
            Arg::new("apiurl")
                .short('a')
                .long("apiurl")
                .help("set api url, don't include https (default: co.wuk.sh)"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("path to save files to (default: ~/Downloads/)"),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("url(s) to download from"),
        )
        // video
        .arg(
            Arg::new("quality")
                .short('q')
                .long("quality")
                .help("set video quality (default: 1080p, other: 4320p+, 2160p, 720p, 480p, 360p)"),
        )
        .arg(
            Arg::new("codec")
                .short('c')
                .long("codec")
                .help("set youtube codec (default: h264, other: av1, vp9)"),
        )
        .arg(
            Arg::new("ttwatermark")
                .short('w')
                .long("ttwatermark")
                .num_args(0)
                .help("disable tiktok watermark (default: false)"),
        )
        // audio
        .arg(
            Arg::new("audioformat")
                .short('f')
                .long("audioformat")
                .help("set audio format (default: mp3, other: best, ogg, wav, opus)"),
        )
        .arg(
            Arg::new("dublang")
                .short('d')
                .long("dublang")
                .num_args(0)
                .help("dub language (default: false)"),
        )
        .arg(
            Arg::new("fullaudio")
                .short('k')
                .long("fullaudio")
                .num_args(0)
                .help("get tiktok full audio (default: false)"),
        )
        .arg(
            Arg::new("mute")
                .short('j')
                .long("mute")
                .num_args(0)
                .help("mute audio when possible (default: false)"),
        )
        .arg(
            Arg::new("playlist")
                .short('y')
                .long("playlist")
                .num_args(0)
                .help("is url path to a yt playlist (default: false)"),
        )
        .get_matches();

    let homedirpathbuf = dirs::home_dir();
    let homedirexpect = homedirpathbuf.expect("method not found in `Option<PathBuf>`");
    let homedir = homedirexpect.display();

    // download mode
    let mut mode = "unspecified".to_string(); // will be changed or throw error & stop, this is so DEBUG has a fallback
    if matches.get_one::<String>("mode").is_none() {
        errors::create_end("you didn't specify a mode");
    } else {
        mode = matches.get_one::<String>("mode").unwrap().to_string();
    }

    // api url
    let d_apiurl = "co.wuk.sh".to_string();
    let apiurl: &String = matches.get_one::<String>("apiurl").unwrap_or(&d_apiurl);

    // save path
    let d_path = format!("{homedir}/Downloads").to_string();
    let path: &String = matches.get_one::<String>("path").unwrap_or(&d_path);

    // video url
    let mut url = "unspecified".to_string();
    if matches.get_one::<String>("url").is_none() {
        errors::create_end("you didn't specify a video url");
    } else {
        url = matches.get_one::<String>("url").unwrap().to_string();
    }

    // video quality
    let d_quality = "1080p".to_string();
    let quality: &String = matches.get_one::<String>("quality").unwrap_or(&d_quality);

    // yt codec
    let d_codec = "h264".to_string();
    let codec: &String = matches.get_one::<String>("codec").unwrap_or(&d_codec);

    // tiktok watermark
    let mut ttwatermark = false;
    if matches.get_flag("ttwatermark") {
        ttwatermark = true;
    } else {
        ttwatermark = false;
    }

    // audio format
    let d_audioformat = "mp3".to_string();
    let audioformat: &String = matches
        .get_one::<String>("audioformat")
        .unwrap_or(&d_audioformat);

    // dub lang
    let mut dublang = false;
    if matches.get_flag("dublang") {
        dublang = true;
    } else {
        dublang = false;
    }

    // full audio
    let mut fullaudio = false;
    if matches.get_flag("fullaudio") {
        fullaudio = true;
    } else {
        fullaudio = false;
    }

    // mute audio
    let mut mute = false;
    if matches.get_flag("mute") {
        mute = true;
    } else {
        mute = false;
    }

    // mute audio
    let mut playlist = false;
    if matches.get_flag("playlist") {
        playlist = true;
    } else {
        playlist = false;
    }

    // sillyyyy :3
    if DEBUG {
        println!(" ");
        println!("{PREFIX} {}", "====[ debug ]====".color("cyan"));
        println!("{PREFIX} **mode**: {mode};");
        println!("{PREFIX} apiurl: {apiurl}; path: {path}; url: {url};");
        println!("{PREFIX} quality: {quality}; codec: {codec};");
        println!(
            "{PREFIX} ttwatermark: {ttwatermark}; audioformat: {audioformat}; dublang: {dublang};"
        );
        println!("{PREFIX} fullaudio: {fullaudio}; mute: {mute};");
        println!("{PREFIX} {}", "====[ debug ]====".color("cyan"));
        println!(" ");
    }

    // now its download time
    if playlist {
        playlist::download(
            PREFIX,
            DEBUG,
            &mode,
            &apiurl,
            &path,
            &url,
            &quality,
            &codec,
            ttwatermark,
            &audioformat,
            dublang,
            fullaudio,
            mute,
        );
    } else {
        if mode == "auto" {
            download::auto(
                PREFIX,
                DEBUG,
                &apiurl,
                &path,
                &url,
                &quality,
                &codec,
                ttwatermark,
                &audioformat,
                dublang,
                fullaudio,
                mute,
            );
        } else if mode == "audio" {
            download::audio(
                PREFIX,
                DEBUG,
                &apiurl,
                &path,
                &url,
                &quality,
                &codec,
                ttwatermark,
                &audioformat,
                dublang,
                fullaudio,
                mute,
            );
        } else {
            errors::create_end("invalid mode. options: auto, audio");
        }
    }
}