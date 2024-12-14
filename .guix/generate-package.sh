#!/bin/sh

cat <<EOF
(add-to-load-path (dirname (current-filename)))

(define-module (utf-nate-package)
	#:use-module (guix)
	#:use-module (guix git-download)
	#:use-module ((guix licenses) #:prefix license:)
	#:use-module (guix build-system cargo)
	#:use-module (gnu packages autotools)
	#:use-module (gnu packages pkg-config)
	#:use-module (gnu packages tls)
	#:use-module (gnu packages video)
	#:use-module (gnu packages xiph)
	#:use-module (gnu packages crates-io)
	#:use-module (gnu packages crates-web)
	#:use-module (gnu packages crates-crypto)
	#:use-module (utf-nate-import)
)

(define (dbg x)
	(display x)
	(display "\n")
	x
)

(define (source-dir)
  (dirname (dirname (dirname (current-filename))))
)

(define vcs-file?
	(or
		;; if we're in a git checkout and the file is not ignored
		(git-predicate (source-dir))
		;; if we're not in a git checkout always return true
		(const #t)
	)
)
EOF

printf '
(define-public utf-nate
	(package
		(name %s)
		(version %s)
		(source
			(local-file
				(source-dir)
				"utf-nate-checkout"
				#:recursive? #t
				#:select? vcs-file?
			)
		)
		(build-system cargo-build-system)
		(arguments
			`(
				#:phases (modify-phases %%standard-phases
					(delete '\''package)
				)
				#:tests? #f
				#:cargo-inputs (
'\
	"$(cargo read-manifest | jq '.name')"\
	"$(cargo read-manifest | jq '.version')"

cargo read-manifest\
	| jq '.dependencies | map(.name + " " + .req)'\
	| sed '1d;$d;s#^ *"#rust-#;s#",$##'\
	| sed 's# \([\^=]\)\(0\.[0-9]*\|[1-9][0-9]*\).*$#\1\2#;s#_#-#g;'\
	| sed 's#^\(.*\)[\^=]\([^-]*\)$#\t\t\t\t\t("\1" ,\1-\2)#'

printf \
'				)
			)
		)
		(native-inputs
			(list
				autoconf
				openssl
				opus
				pkg-config
			)
		)
		(inputs
			(list
				yt-dlp
			)
		)
		(synopsis %s)
		(description "%s")
		(home-page %s)
		(license license:lgpl3+)
	)
)

utf-nate
'\
	"$(cargo read-manifest | jq '.description')"\
	""\
	"$(cargo read-manifest | jq '.repository')"
