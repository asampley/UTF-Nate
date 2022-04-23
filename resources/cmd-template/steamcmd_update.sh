#!/bin/sh
set -euf

id=$1
dir=$2

if steamcmd +force_install_dir "${dir}" +login anonymous +app_update "${id}" validate +quit 1>&2; then
	echo "Update finished successfully"
else
	echo "Update failed"
fi
