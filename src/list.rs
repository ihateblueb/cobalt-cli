use crate::download;

pub fn start(
    prefix: &str,
    debug: bool,
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
    println!("{} starting to download list content... you might be here for a while!", prefix);

    for url in json["skins"].as_array().ok_or(DataParseError::SkinsNotAnArray)? {
        // ...
    }


    println!("{} finished downloading from list.", prefix);
}