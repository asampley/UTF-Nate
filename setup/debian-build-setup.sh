#!/bin/sh
set -euf

# NOTE
# This is not meant to be an automatic process, and will prompt for acceptance of the apt packages.
# Confirm yourself that it's okay, or pipe in yes to skip

# Install packages required for building
sudo apt install build-essential pkg-config libssl-dev autoconf libtool cmake
