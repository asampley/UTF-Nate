#!/bin/sh
set -euf

service="$1"
mode="$2"

case "${mode}" in
	start)
		success="Started ${service}"

		if service "${service}" status > /dev/null 2>&1; then
			echo "${service} already running"
			exit
		fi
		;;
	restart)
		success="Restarted ${service}"

		if ! service "${service}" status > /dev/null 2>&1; then
			echo "${service} is not running"
			exit
		fi

		mode="--full-restart"
		;;
	stop)
		success="Stopped ${service}"

		if ! service "${service}" status > /dev/null 2>&1; then
			echo "${service} is not running"
			exit
		fi
		;;
	*)
		echo "Script error, please ask the bot maintainer to fix the script"
		exit
		;;
esac

if sudo service "${service}" "${mode}"; then
	echo "${success}"
else
	echo "Error occurred when ${mode}ing ${service}"
fi
