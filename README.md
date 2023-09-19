# cobalt-cli
save what you love from your terminal
## how to get
from the source code you can build with `cargo build --release`. or, go grab a binary from releases. theres currently a mac one, but i don't know how to run it on mac. you should probably build it yourself.
## how to use
> [tocobaltcli] is the path or alias to the cobalt cli executable  

for a list of all commands, you can run `[tocobaltcli] --help`, and it's similar to the below:  

| flags                            | description                                                                               |
|----------------------------------|-------------------------------------------------------------------------------------------|
|  -h, --help                      | print help                                                                                |
|  -V, --version                   | print version                                                                             |
|  -m, --mode <mode>               | set which mode to download with (default: auto, other: audio)                             |
|  -a, --apiurl <apiurl>           | set api url, don't include https (default: co.wuk.sh)                                     |
|  -p, --path <path>               | path to save files to (default: ~/Downloads/)                                             |
|  -u, --url <url>                 | url to download from                                                                      |
|  -q, --quality <quality>         | set video quality (default: 1080p, other: 4320p+, 2160p, 720p, 480p, 360p), 480p, 360p)   |
|  -c, --codec <codec>             | set youtube codec (default: h264, other: av1, vp9)                                        |
|  -w, --ttwatermark               | disable tiktok watermark (default: false)                                                 |
|  -f, --audioformat <audioformat> | set audio format (default: mp3, other: best, ogg, wav, opus)                              |
|  -d, --dublang                   | dub language (default: false)                                                             |
|  -k, --fullaudio                 | get tiktok full audio (default: false)                                                    |
|  -j, --mute                      | mute audio when possible (default: false)                                                 |
