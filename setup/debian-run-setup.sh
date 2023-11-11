<<<<<<< HEAD:setup/debian-setup.sh
#!/bin/sh
set -euf

# NOTE
# This is not meant to be an automatic process, and will prompt for acceptance of the apt packages.
# Confirm yourself that it's okay, or pipe in yes to skip

# Add one of these youtube-dl programs to your install.
# These require python.
sudo apt install python

## youtube-dl install
#sudo curl -L https://yt-dl.org/downloads/latest/youtube-dl -o /usr/local/bin/youtube-dl
#sudo chmod a+rx /usr/local/bin/youtube-dl

# yt-dlp install
sudo curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
sudo chmod a+rx /usr/local/bin/yt-dlp

## youtube-dlc install
#sudo curl -L https://github.com/blackjack4494/yt-dlc/releases/latest/download/youtube-dlc -o /usr/local/bin/youtube-dlc
#sudo chmod a+rx /usr/local/bin/youtube-dlc
