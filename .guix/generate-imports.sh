#!/bin/sh

echo \
'(define-module (utf-nate-import)
	#:use-module (srfi srfi-9 gnu)
	#:use-module (guix)
	#:use-module ((guix licenses) #:prefix license:)
	#:use-module (guix build-system cargo)
	#:use-module (gnu packages crates-crypto)
	#:use-module (gnu packages crates-io)
	#:use-module (gnu packages crates-tls)
	#:use-module (gnu packages crates-web)
	#:use-module (gnu packages crates-windows)
)

(define unknown-license!
	(set-fields license:expat
		((license:license-name) "Unknown")
		((license:license-uri) "Unknown")
		((license:license-comment) "Unknown")
	)
)
'

cargo read-manifest\
	| jq '.dependencies | map(.name + .req)'\
	| sed '1d;$d;s#[ ",]##g;'\
	| while IFS= read -r line; do
		name=$(echo "$line" | sed 's#\^.*##')
		cargo_min=$(echo "$line" | sed 's#.*\^##')
		major=$(
			echo "$line"\
				| sed 's#^[^^]*\^##;s#\(0\.[0-9]*\|[1-9][0-9]*\)\(\.[0-9]*\)*$#\1#'
		)

		guix_best=$(
			guix show "$(echo "rust-$name@$major" | sed 's#_#-#g;')" 2>/dev/null \
				| sed "/version/!d;s#version: ##"\
				| sort -rV\
				| head -n1\
				|| echo 0
		)

		if [ "$(printf '%s\n' "$guix_best" "$cargo_min" | sort -V | head -n1)" != "$cargo_min" ]; then
			echo "$name@$major"
		 fi
	done\
	| xargs -tL1 guix import crate --recursive

#cargo read-manifest\
#	| jq '.dependencies | map(.name + .req)'\
#	| sed '1d;$d;s#[ ",]##g;'\
#	| xargs -tI{} sh -c 'guix show "rust-$(echo "{}" | sed '\''s#_#-#g;s#\^\(0\.[0-9]*\|[1-9][0-9]*\)\(\.[0-9]*\)*$#@\1#'\'')" | sed "/version/!d;s#version: ##" | sort -rV | head -n1'\
#	#| xargs -L1 guix import crate --recursive
