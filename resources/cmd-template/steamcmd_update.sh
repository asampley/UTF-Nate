#!/bin/sh
set -euf

id=$1
shift

if /usr/games/steamcmd "$@" +login anonymous +app_update "${id}" validate +quit 1>&2; then
	echo "Update finished successfully"
else
	echo "Update failed"
fi
