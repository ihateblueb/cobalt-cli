#!/bin/bash
# cobalt-cli
VERSION="0.1"
DEBUG="false"

# default api url
APIURL="co.wuk.sh"
# default quality
QUALITY="1080p"
# default codec
YOUTUBECODEC="h264"
# default vimeo download type
VIMEODOWNLOADSTYPE="progressive"
# default tiktok watermark toggle
DISABLETIKTOKWATERMARK="false"
# default audio format
AUDIOFORMAT="mp3"
# default tiktok full audio toggle
TIKTOKFULLAUDIO="false"
# default audio mute
MUTEAUDIO="false"
# default language track
DUBLANG="false"
# default download path
DOWNLOADPATH="/home/$USER/Downloads"

# video url will be filled in
VIDEOURL=""

show_help() {
  echo "cobalt-cli v$VERSION"
  echo "Usage: $0 [mode] [flags]"
  echo "Commands:"
  echo "  auto                get a video from cobalt in auto mode"
  echo "  audio               get a video from cobalt in audio mode"
  echo "Flags:"
  echo "  -a (api url)        set api url, dont include https (default: co.wuk.sh)"
  echo "  -p (path)           path to save file to (default: ~/)"
  echo "  -u (url)            url to download from (must be last)"
  echo " "
  echo "  -q (quality)        set video quality (default: 1080p, other: 4320p+, 2160p, 720p, 480p, 360p)"
  echo "  -c (youtube codec)  set youtube codec (default: h264, other: av1, vp9)"
  echo "  -v (download type)  vimeo downloads type (default: progressive, other: dash)"
  echo "  -w                  disable tiktok watermark (default: false)"
  echo " "
  echo "  -f (format)         set audio format (default: mp3, other: best, ogg, wav, opus)"
  echo "  -d                  dub language (default: false)"
  echo "  -k                  full audio (default: false)"
  echo "  -m                  mute audio when possible (default: false)"
}

mode_specified="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
  -a)
    APIURL="$2"
    shift 2
    ;;
  -q)
    QUALITY="$2"
    shift 2
    ;;
  -c)
    YOUTUBECODEC="$2"
    shift 2
    ;;
  -v)
    VIMEODOWNLOADSTYPE="$2"
    shift 2
    ;;
  -w)
    DISABLETIKTOKWATERMARK="true"
    shift 1
    ;;
  -f)
    AUDIOFORMAT="$2"
    shift 2
    ;;
  -d)
    DUBLANG="true"
    shift 1
    ;;
  -k)
    TIKTOKFULLAUDIO="true"
    shift 1
    ;;
  -m)
    MUTEAUDIO="true"
    shift 1
    ;;
  -u)
    VIDEOURL="$2"
    shift 2
    ;;
  -p)
    DOWNLOADPATH="$2"
    shift 2
    ;;
  -h)
    show_help
    exit 0
    ;;
  --help)
    show_help
    exit 0
    ;;
  auto | audio)
    if [ "$mode_specified" == "false" ]; then
      MODE="$1"
      mode_specified="true"
      shift
    else
      echo "[cobalt] multiple modes specified, use only one mode."
      exit 1
    fi
    ;;
  *)
    echo "[cobalt] unrecognized argument: $1"
    exit 1
    ;;
  esac
done

if [ "$mode_specified" == "false" ]; then
  echo "[cobalt] mode not specified. see -h for help."
  exit 1
fi

download() {

  if [[ "$DEBUG" == "true" ]]; then
    echo "[debug] $APIURL - $VIDEOURL - $QUALITY - $YOUTUBECODEC - $VIMEODOWNLOADSTYPE - $DISABLETIKTOKWATERMARK - $AUDIOFORMAT - $DUBLANG - $TIKTOKFULLAUDIO - $MUTEAUDIO - $MODE"
  fi

  if [[ "$MODE" == "auto" ]]; then
    APIRESPONSE=$(curl -sb -X POST https://$APIURL/api/json -H "Content-Type: application/json" -H "Accept: application/json" -d "{\"url\":\"$VIDEOURL\",\"vCodec\":\"$YOUTUBECODEC\",\"vQuality\":\"$QUALITY\",\"aFormat\":\"$AUDIOFORMAT\",\"isNoTTWatermark\":\"$DISABLETIKTOKWATERMARK\",\"isTTFullAudio\":\"$TIKTOKFULLAUDIO\",\"dubLang\":\"$DUBLANG\"}")
    if [[ $(jq -r .'status' <<<"$APIRESPONSE") == "stream" ]]; then
      echo "[cobalt] downloading video to $DOWNLOADPATH..."
      cd $DOWNLOADPATH
      curl -O -J $(jq -r .'url' <<<"$APIRESPONSE")
      cd - >/dev/null
      echo "[cobalt] complete!"
      exit 0
    else
      echo "[cobalt] uh-oh... $(jq -r .'text' <<<"$APIRESPONSE")"
      exit 1
    fi
  else 
    APIRESPONSE=$(curl -sb -X POST https://$APIURL/api/json -H "Content-Type: application/json" -H "Accept: application/json" -d "{\"url\":\"$VIDEOURL\",\"vCodec\":\"$YOUTUBECODEC\",\"vQuality\":\"$QUALITY\",\"aFormat\":\"$AUDIOFORMAT\",\"isAudioOnly\":\"true\",\"isNoTTWatermark\":\"$DISABLETIKTOKWATERMARK\",\"isTTFullAudio\":\"$TIKTOKFULLAUDIO\",\"isAudioMuted\":\"$MUTEAUDIO\",\"dubLang\":\"$DUBLANG\"}")
    if [[ $(jq -r .'status' <<<"$APIRESPONSE") == "stream" ]]; then
      echo "[cobalt] downloading video to $DOWNLOADPATH..."
      cd $DOWNLOADPATH
      curl -O -J $(jq -r .'url' <<<"$APIRESPONSE")
      cd - >/dev/null
      echo "[cobalt] complete!"
      exit 0
    else
      echo "[cobalt] uh-oh... $(jq -r .'text' <<<"$APIRESPONSE")"
      exit 1
    fi
  fi
  
}

if [[ "$MODE" == "auto" ]]; then
  if [[ "$VIDEOURL" != "" ]]; then
    echo "[cobalt] fetching stream url from $APIURL in auto mode..."
    download
  else
    echo "[cobalt] please specify a url with the -u flag."
  fi
fi

if [[ "$MODE" == "audio" ]]; then
  if [[ "$VIDEOURL" != "" ]]; then
    echo "[cobalt] fetching stream url from $APIURL in audio mode..."
    download
  else
    echo "[cobalt] please specify a url with the -u flag."
  fi
fi
