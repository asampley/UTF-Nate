#!/bin/sh
set -euf

mode=$1
service=$2

case "${mode}" in
	start)
		success="Started ${service}"

		if systemctl is-active "${service}" | grep "^active$" > /dev/null 2>&1; then
			echo "${service} already running"
			exit
		fi
		;;
	stop)
		success="Stopped ${service}"

		if systemctl is-active valheim | grep "^inactive$" > /dev/null 2>&1; then
			echo "${service} is not running"
			exit
		fi
		;;
	*)
		echo "Script error, please ask the bot maintainer to fix the script"
		exit
		;;
esac

sudo systemctl "${mode}" "${service}" 

echo "${success}"
