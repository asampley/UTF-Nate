#!/bin/sh
set -euf

# NOTE
# This is not meant to be an automatic process, and will prompt for acceptance of the apt packages.
# Confirm yourself that it's okay, or pipe in yes to skip

# yt-dlp requires python.
apt install python3

# yt-dlp install
curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
chmod a+rx /usr/local/bin/yt-dlp

