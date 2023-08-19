#!/bin/sh
set -euf

mode=$1
service=$2

case "${mode}" in
	start)
		success="Started ${service}"

		if systemctl is-active "${service}" > /dev/null 2>&1; then
			echo "${service} already running"
			exit
		fi
		;;
	restart)
		success="Restarted ${service}"

		if ! systemctl is-active "${service}" > /dev/null 2>&1; then
			echo "${service} is not running"
			exit
		fi
		;;
	stop)
		success="Stopped ${service}"

		if ! systemctl is-active "${service}" > /dev/null 2>&1; then
			echo "${service} is not running"
			exit
		fi
		;;
	*)
		echo "Script error, please ask the bot maintainer to fix the script"
		exit
		;;
esac

if sudo systemctl "${mode}" "${service}"; then
	echo "${success}"
else
	echo "Error occurred when ${mode}ing ${service}"
fi
