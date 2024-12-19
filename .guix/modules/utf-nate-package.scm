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
	#:use-module (gnu packages crates-crypto)
	#:use-module (gnu packages crates-io)
	#:use-module (gnu packages crates-web)
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

(define-public utf-nate
	(package
		(name "utf-nate")
		(version "0.4.0")
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
				#:phases (modify-phases %standard-phases
					(delete 'package)
				)
				#:tests? #f
				#:cargo-inputs (
					("rust-askama" ,rust-askama-0.12)
					("rust-async-trait" ,rust-async-trait-0.1)
					("rust-axum" ,rust-axum-0.6)
					("rust-axum-extra" ,rust-axum-extra-0.7)
					("rust-caith" ,rust-caith-4)
					("rust-chrono" ,rust-chrono-0.4)
					("rust-clap" ,rust-clap-4)
					("rust-dashmap" ,rust-dashmap-5)
					("rust-futures" ,rust-futures-0.3)
					("rust-fxhash" ,rust-fxhash-0.2)
					("rust-hyper" ,rust-hyper-0.14)
					("rust-iso8601-duration" ,rust-iso8601-duration-0.2)
					("rust-itertools" ,rust-itertools-0.11)
					("rust-markdown" ,rust-markdown-1)
					("rust-nom" ,rust-nom-7)
					("rust-once-cell" ,rust-once-cell-1)
					("rust-poise" ,rust-poise-0.6)
					("rust-public-ip" ,rust-public-ip-0.2)
					("rust-rand" ,rust-rand-0.8)
					("rust-regex" ,rust-regex-1)
					("rust-reqwest" ,rust-reqwest-0.11)
					("rust-ring" ,rust-ring-0.16)
					("rust-serde" ,rust-serde-1)
					("rust-serde-json" ,rust-serde-json-1)
					("rust-serde-urlencoded" ,rust-serde-urlencoded-0.7)
					("rust-serde-with" ,rust-serde-with-3)
					("rust-serenity" ,rust-serenity-0.12)
					("rust-songbird" ,rust-songbird-0.4)
					("rust-sqlx" ,rust-sqlx-0.8)
					("rust-symphonia" ,rust-symphonia-0.5)
					("rust-thiserror" ,rust-thiserror-1)
					("rust-tokio" ,rust-tokio-1)
					("rust-toml" ,rust-toml-0.7)
					("rust-tower-http" ,rust-tower-http-0.4)
					("rust-tracing" ,rust-tracing-0.1)
					("rust-tracing-subscriber" ,rust-tracing-subscriber-0.3)
					("rust-triple-accel" ,rust-triple-accel-0.4)
					("rust-uuid" ,rust-uuid-1)
					("rust-walkdir" ,rust-walkdir-2)
				)
			)
		)
		(native-inputs
			(list
				opus
				pkg-config
			)
		)
		(inputs
			(list
				yt-dlp
			)
		)
		(synopsis "A discord bot for playing music")
		(description "")
		(home-page "https://github.com/asampley/UTF-Nate")
		(license license:lgpl3+)
	)
)

utf-nate
