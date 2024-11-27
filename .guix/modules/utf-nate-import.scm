(define-module (utf-nate-import)
	#:use-module (guix)
	#:use-module ((guix licenses) #:prefix license:)
	#:use-module (guix build-system cargo)
	#:use-module (gnu packages crates-crypto)
	#:use-module (gnu packages crates-io)
	#:use-module (gnu packages crates-tls)
	#:use-module (gnu packages crates-web)
	#:use-module (gnu packages crates-windows)
)

(define-public rust-caith-4
  (package
    (name "rust-caith")
    (version "4.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "caith" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1gx68sgxsscgkavjrxzbsw8q86rba88bx6bk8f4rwdg6m4kfj3c6"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-pest" ,rust-pest-2)
                       ("rust-pest-derive" ,rust-pest-derive-2)
                       ("rust-rand" ,rust-rand-0.8))
       #:cargo-development-inputs (("rust-rand-core" ,rust-rand-core-0.6))))
    (home-page "https://github.com/Geobert/caith")
    (synopsis "dice roller library supporting many features")
    (description
     "This package provides a dice roller library supporting many features.")
    (license license:expat)))

(define-public rust-swc-plugin-macro-0.9
  (package
    (name "rust-swc-plugin-macro")
    (version "0.9.16")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_plugin_macro" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1x3l6ldnc65ni5xj98jgl0dymh4n8035q58an8vhc1w42i4dncij"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Macro support for authoring plugin's transform fn")
    (description
     "This package provides Macro support for authoring plugin's transform fn.")
    (license license:asl2.0)))

(define-public rust-swc-plugin-0.90
  (package
    (name "rust-swc-plugin")
    (version "0.90.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_plugin" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1izw97n7kldybppfm6jksivrj2kn7lv3zkp4igfqrvl3ls1spllx"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-once-cell" ,rust-once-cell-1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "SDK for authoring swc plugin")
    (description "This package provides SDK for authoring swc plugin.")
    (license license:asl2.0)))

(define-public rust-swc-nodejs-common-0.0.5
  (package
    (name "rust-swc-nodejs-common")
    (version "0.0.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_nodejs_common" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "013bca6qm5lpcjnr1fvn4xgpracnzkpjxvfcg91ssankz4g8f02w"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-napi" ,rust-napi-2)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-tracing-subscriber" ,rust-tracing-subscriber-0.3))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Common code for SWC bindings")
    (description "This package provides Common code for SWC bindings.")
    (license license:asl2.0)))

(define-public rust-swc-node-bundler-0.47
  (package
    (name "rust-swc-node-bundler")
    (version "0.47.48")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_node_bundler" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0kzm6y6hgsc0qaysdj8wwqa22yx64cmvmffi69lqns2yaz0jkjvz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-is-macro" ,rust-is-macro-0.2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-string-enum" ,rust-string-enum-0.4)
                       ("rust-swc" ,rust-swc-0.260)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-bundler" ,rust-swc-bundler-0.213)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-codegen" ,rust-swc-ecma-codegen-0.138)
                       ("rust-swc-ecma-loader" ,rust-swc-ecma-loader-0.43)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-transforms" ,rust-swc-ecma-transforms-0.217)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-swc-node-base" ,rust-swc-node-base-0.5)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Speedy web compiler")
    (description "This package provides Speedy web compiler.")
    (license license:asl2.0)))

(define-public rust-tikv-jemalloc-sys-0.5
  (package
    (name "rust-tikv-jemalloc-sys")
    (version "0.5.4+5.3.0-patched")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tikv-jemalloc-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1lc5vm1p9dqdvd3mn3264zddnd7z6i95ch3y69prnjgxp0y480ll"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-libc" ,rust-libc-0.2))))
    (home-page "https://github.com/tikv/jemallocator")
    (synopsis "Rust FFI bindings to jemalloc")
    (description "This package provides Rust FFI bindings to jemalloc.")
    (license (list license:expat license:asl2.0))))

(define-public rust-tikv-jemallocator-0.5
  (package
    (name "rust-tikv-jemallocator")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tikv-jemallocator" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jpanfm9az8hcbg6dyxdabykx03lj0j4g9cbwfa6rig5dg1f0pwn"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-libc" ,rust-libc-0.2)
                       ("rust-tikv-jemalloc-sys" ,rust-tikv-jemalloc-sys-0.5))))
    (home-page "https://github.com/tikv/jemallocator")
    (synopsis "Rust allocator backed by jemalloc")
    (description "This package provides a Rust allocator backed by jemalloc.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mimalloc-rust-sys-1
  (package
    (name "rust-mimalloc-rust-sys")
    (version "1.7.9-source")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mimalloc-rust-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1nymbp7av46h09aig9n153sjqmwlcv53w4v82llrz05984rf24v4"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-cty" ,rust-cty-0.2))))
    (home-page "")
    (synopsis "mimalloc_rust hand writted sys binding")
    (description
     "This package provides mimalloc_rust hand writted sys binding.")
    (license license:expat)))

(define-public rust-mimalloc-rust-0.2
  (package
    (name "rust-mimalloc-rust")
    (version "0.2.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mimalloc-rust" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1glvzha0k0rg275ri7lxgf0cydmywi880vf4n8841ywf5742ddsy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cty" ,rust-cty-0.2)
                       ("rust-mimalloc-rust-sys" ,rust-mimalloc-rust-sys-1))))
    (home-page "https://github.com/lemonhx/mimalloc-rust")
    (synopsis "the best binding for mimalloc in rust")
    (description
     "This package provides the best binding for mimalloc in rust.")
    (license license:expat)))

(define-public rust-swc-node-base-0.5
  (package
    (name "rust-swc-node-base")
    (version "0.5.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_node_base" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0i83zcwc4airwsc2qdj3y6v076avic3f20q2x69x2gqq9cgrx584"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-mimalloc-rust" ,rust-mimalloc-rust-0.2)
                       ("rust-tikv-jemallocator" ,rust-tikv-jemallocator-0.5))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Speedy web compiler")
    (description "This package provides Speedy web compiler.")
    (license license:asl2.0)))

(define-public rust-testing-macros-0.2
  (package
    (name "rust-testing-macros")
    (version "0.2.14")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "testing_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "01pflhviws0l375zazhklgfl08gyg8dv1g7qbxny9bqn04vn15m3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-glob" ,rust-glob-0.3)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-relative-path" ,rust-relative-path-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "General purpose testing macros")
    (description "This package provides General purpose testing macros.")
    (license license:asl2.0)))

(define-public rust-testing-0.33
  (package
    (name "rust-testing")
    (version "0.33.25")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "testing" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "09blpkrypk65xkx5jy19afqc5glr8qmsjynkmgkhh8mdlnm2swhw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ansi-term" ,rust-ansi-term-0.12)
                       ("rust-cargo-metadata" ,rust-cargo-metadata-0.15)
                       ("rust-difference" ,rust-difference-2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-pretty-assertions" ,rust-pretty-assertions-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-error-reporters" ,rust-swc-error-reporters-0.15)
                       ("rust-testing-macros" ,rust-testing-macros-0.2)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-tracing-subscriber" ,rust-tracing-subscriber-0.3))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Testing utilities for the swc project")
    (description
     "This package provides Testing utilities for the swc project.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-testing-0.20
  (package
    (name "rust-swc-ecma-testing")
    (version "0.20.19")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_testing" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1cll1hrqxh415kqvv5p94r65190byxfrnng71xgdylc8rvab9r9g"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-hex" ,rust-hex-0.4)
                       ("rust-sha-1" ,rust-sha-1-0.10)
                       ("rust-testing" ,rust-testing-0.33)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Testing utilities for ecmascript")
    (description "This package provides Testing utilities for ecmascript.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-testing-0.129
  (package
    (name "rust-swc-ecma-transforms-testing")
    (version "0.129.20")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_testing" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0zw287dlzmc5mxcjx05azchmm7xqwnyzn42jy5bv03ifll9dqavk"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ansi-term" ,rust-ansi-term-0.12)
                       ("rust-anyhow" ,rust-anyhow-1)
                       ("rust-base64" ,rust-base64-0.13)
                       ("rust-hex" ,rust-hex-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sha-1" ,rust-sha-1-0.10)
                       ("rust-sourcemap" ,rust-sourcemap-6)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-codegen" ,rust-swc-ecma-codegen-0.138)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-testing" ,rust-swc-ecma-testing-0.20)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-tempfile" ,rust-tempfile-3)
                       ("rust-testing" ,rust-testing-0.33))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "rust port of babel and closure compiler")
    (description
     "This package provides rust port of babel and closure compiler.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-quote-macros-0.44
  (package
    (name "rust-swc-ecma-quote-macros")
    (version "0.44.14")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_quote_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0xgrl10c4422x71sylavx4fgnnyapp2g17d5gwidhsfwwfy4xnwm"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-pmutil" ,rust-pmutil-0.5)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-macros-common" ,rust-swc-macros-common-0.3)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Quasi quotation system for ecmascript")
    (description
     "This package provides Quasi quotation system for ecmascript.")
    (license license:asl2.0)))

(define-public rust-swc-css-prefixer-0.149
  (package
    (name "rust-swc-css-prefixer")
    (version "0.149.26")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_prefixer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1d67a7v23481a7bd4n3bcpavklx1nfp4zw4hn3gb4m8j262x0sr4"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-once-cell" ,rust-once-cell-1)
                       ("rust-preset-env-base" ,rust-preset-env-base-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137)
                       ("rust-swc-css-utils" ,rust-swc-css-utils-0.134)
                       ("rust-swc-css-visit" ,rust-swc-css-visit-0.136))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Port of stylis")
    (description "This package provides Port of stylis.")
    (license license:asl2.0)))

(define-public rust-swc-css-parser-0.146
  (package
    (name "rust-swc-css-parser")
    (version "0.146.24")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_parser" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0f7qrqllwhnv14f3vah90anp4bpvncgc70y3m3qfl5j3cd0203lv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-lexical" ,rust-lexical-6)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "CSS parser")
    (description "This package provides CSS parser.")
    (license license:asl2.0)))

(define-public rust-swc-css-modules-0.25
  (package
    (name "rust-swc-css-modules")
    (version "0.25.27")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_modules" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "18r01s6qvbiql4rm1ydmgy33fmm9hm2f8crgx18lyym11xahws63"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137)
                       ("rust-swc-css-codegen" ,rust-swc-css-codegen-0.147)
                       ("rust-swc-css-parser" ,rust-swc-css-parser-0.146)
                       ("rust-swc-css-visit" ,rust-swc-css-visit-0.136))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "CSS modules")
    (description "This package provides CSS modules.")
    (license license:asl2.0)))

(define-public rust-swc-css-minifier-0.112
  (package
    (name "rust-swc-css-minifier")
    (version "0.112.24")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_minifier" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0cn53lfrn4cwjzibv4a82ymh8l1smjrwkrsm91nwwh9bhga4dzzr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137)
                       ("rust-swc-css-utils" ,rust-swc-css-utils-0.134)
                       ("rust-swc-css-visit" ,rust-swc-css-visit-0.136))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "CSS minifier")
    (description "This package provides CSS minifier.")
    (license license:asl2.0)))

(define-public rust-swc-css-compat-0.23
  (package
    (name "rust-swc-css-compat")
    (version "0.23.24")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_compat" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0xv909f4chj07vqnic5s396bz18lzgx1ab28d1ciqgnfccq7qpf2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137)
                       ("rust-swc-css-utils" ,rust-swc-css-utils-0.134)
                       ("rust-swc-css-visit" ,rust-swc-css-visit-0.136))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Port of stylis")
    (description "This package provides Port of stylis.")
    (license license:asl2.0)))

(define-public rust-swc-css-visit-0.136
  (package
    (name "rust-swc-css-visit")
    (version "0.136.21")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_visit" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "12aqh5rb6zlk9mcskh7cccdkrkbin0l7n0bsh96mf0xywhl6azm7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137)
                       ("rust-swc-visit" ,rust-swc-visit-0.5))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Visitor for css")
    (description "This package provides Visitor for css.")
    (license license:asl2.0)))

(define-public rust-swc-css-utils-0.134
  (package
    (name "rust-swc-css-utils")
    (version "0.134.21")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_utils" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0mvb6c2b6zl2q99wmndjpr5pwl4cdmkac45my69lwm66fhlwklgy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-once-cell" ,rust-once-cell-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137)
                       ("rust-swc-css-visit" ,rust-swc-css-visit-0.136))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Port of stylis")
    (description "This package provides Port of stylis.")
    (license license:asl2.0)))

(define-public rust-swc-css-codegen-macros-0.2
  (package
    (name "rust-swc-css-codegen-macros")
    (version "0.2.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_codegen_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1mqkpknc13av6z1fl4hkpzc50x0zyk2n95yc3fm8bpmxgn6cwbny"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-swc-macros-common" ,rust-swc-macros-common-0.3)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Internal macro for the css code generator")
    (description
     "This package provides Internal macro for the css code generator.")
    (license license:asl2.0)))

(define-public rust-swc-css-codegen-0.147
  (package
    (name "rust-swc-css-codegen")
    (version "0.147.24")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_codegen" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "116ga04zizyxr0wnjmkghlz61fkqksnqi3fgh3q1acqyp5mbdsla"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-auto-impl" ,rust-auto-impl-1)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137)
                       ("rust-swc-css-codegen-macros" ,rust-swc-css-codegen-macros-0.2)
                       ("rust-swc-css-utils" ,rust-swc-css-utils-0.134))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "CSS code generator for the swc project")
    (description
     "This package provides CSS code generator for the swc project.")
    (license license:asl2.0)))

(define-public rust-auto-impl-1
  (package
    (name "rust-auto-impl")
    (version "1.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "auto_impl" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0hmfcahj0vrnzq7rayk7r428zp54x9a8awgw6wil753pbvqz71rw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/auto-impl-rs/auto_impl/")
    (synopsis
     "Automatically implement traits for common smart pointers and closures")
    (description
     "This package provides Automatically implement traits for common smart pointers and closures.")
    (license (list license:expat license:asl2.0))))

(define-public rust-swc-graph-analyzer-0.20
  (package
    (name "rust-swc-graph-analyzer")
    (version "0.20.25")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_graph_analyzer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1kxbybc3kiw6jr5jv4f6licmkzs8jwd8fpcfpsvgkm43y7128zp5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-auto-impl" ,rust-auto-impl-1)
                       ("rust-petgraph" ,rust-petgraph-0.6)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-fast-graph" ,rust-swc-fast-graph-0.19)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Graph analyzer")
    (description "This package provides Graph analyzer.")
    (license license:asl2.0)))

(define-public rust-swc-bundler-0.213
  (package
    (name "rust-swc-bundler")
    (version "0.213.36")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_bundler" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jq5nz2gvm00sylx1lifzc89qs21jfakldxkq1jqc9974dgy5bw8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-anyhow" ,rust-anyhow-1)
                       ("rust-crc" ,rust-crc-2)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-is-macro" ,rust-is-macro-0.2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-petgraph" ,rust-petgraph-0.6)
                       ("rust-radix-fmt" ,rust-radix-fmt-1)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-relative-path" ,rust-relative-path-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-codegen" ,rust-swc-ecma-codegen-0.138)
                       ("rust-swc-ecma-loader" ,rust-swc-ecma-loader-0.43)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-optimization" ,rust-swc-ecma-transforms-optimization-0.186)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-swc-fast-graph" ,rust-swc-fast-graph-0.19)
                       ("rust-swc-graph-analyzer" ,rust-swc-graph-analyzer-0.20)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Very fast ecmascript bundler")
    (description "This package provides Very fast ecmascript bundler.")
    (license license:asl2.0)))

(define-public rust-serde-derive-internals-0.29
  (package
    (name "rust-serde-derive-internals")
    (version "0.29.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serde_derive_internals" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "04g7macx819vbnxhi52cx0nhxi56xlhrybgwybyy7fb9m4h6mlhq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://serde.rs")
    (synopsis "AST representation used by Serde derive macros. Unstable")
    (description
     "This package provides AST representation used by Serde derive macros.  Unstable.")
    (license (list license:expat license:asl2.0))))

(define-public rust-schemars-derive-0.8
  (package
    (name "rust-schemars-derive")
    (version "0.8.21")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "schemars_derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03ncmrkldfmdc9skmlyysx2vqdlyyz91r5mbavw77zwaay4fbvmi"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-serde-derive-internals" ,rust-serde-derive-internals-0.29)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://graham.cool/schemars/")
    (synopsis "Macros for #[derive(JsonSchema)], for use with schemars")
    (description
     "This package provides Macros for #[derive(@code{JsonSchema})], for use with schemars.")
    (license license:expat)))

(define-public rust-schemars-0.8
  (package
    (name "rust-schemars")
    (version "0.8.21")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "schemars" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "14lyx04388wgbilgcm0nl75w6359nw16glswfqv7x2rpi9329h09"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.5)
                       ("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-bigdecimal" ,rust-bigdecimal-0.4)
                       ("rust-bigdecimal" ,rust-bigdecimal-0.3)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-dyn-clone" ,rust-dyn-clone-1)
                       ("rust-either" ,rust-either-1)
                       ("rust-enumset" ,rust-enumset-1)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-rust-decimal" ,rust-rust-decimal-1)
                       ("rust-schemars-derive" ,rust-schemars-derive-0.8)
                       ("rust-semver" ,rust-semver-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-smol-str" ,rust-smol-str-0.1)
                       ("rust-url" ,rust-url-2)
                       ("rust-uuid" ,rust-uuid-0.8)
                       ("rust-uuid" ,rust-uuid-1))))
    (home-page "https://graham.cool/schemars/")
    (synopsis "Generate JSON Schemas from Rust code")
    (description "This package provides Generate JSON Schemas from Rust code.")
    (license license:expat)))

(define-public rust-wcgi-host-0.1
  (package
    (name "rust-wcgi-host")
    (version "0.1.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wcgi-host" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1k4mnrikqni8838cyf7rpvwc1961k2in67arzfia52fk1qmwyqm7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-http" ,rust-http-0.2)
                       ("rust-schemars" ,rust-schemars-0.8)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-wasmparser" ,rust-wasmparser-0.95)
                       ("rust-wcgi" ,rust-wcgi-0.1))))
    (home-page "https://wasmer.io/")
    (synopsis
     "Utilities for implementing WCGI (Webassembly Common Gateway Interface) support in hosts")
    (description
     "This package provides Utilities for implementing WCGI (Webassembly Common Gateway Interface) support
in hosts.")
    (license license:expat)))

(define-public rust-http-serde-1
  (package
    (name "rust-http-serde")
    (version "1.1.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "http-serde" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1vnald3g10gxj15dc5jjjk7aff23p1zly0xgzhn5gwfrb9k0nmkg"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-http" ,rust-http-0.2)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://lib.rs/crates/http-serde")
    (synopsis
     "Serde support for the http crate. (De)serialize HeaderMap, Uri, Method, StatusCode")
    (description
     "This package provides Serde support for the http crate. (De)serialize @code{HeaderMap}, Uri, Method,
@code{StatusCode}.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-wcgi-0.1
  (package
    (name "rust-wcgi")
    (version "0.1.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wcgi" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0xxzwaiia3yjmkqi3vhrsa8qcns19z3i59i5pnbq2fpc9qrqzjir"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-http" ,rust-http-0.2)
                       ("rust-http-serde" ,rust-http-serde-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-url" ,rust-url-2))))
    (home-page "https://wasmer.io/")
    (synopsis "Common abstractions for defining a WCGI server")
    (description
     "This package provides Common abstractions for defining a WCGI server.")
    (license license:expat)))

(define-public rust-wai-bindgen-rust-impl-0.2
  (package
    (name "rust-wai-bindgen-rust-impl")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wai-bindgen-rust-impl" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "197ydqvia8zyd79s28mml9dw0396xzkj7qd34n2dwih22z0vbvmx"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-syn" ,rust-syn-1)
                       ("rust-wai-bindgen-gen-core" ,rust-wai-bindgen-gen-core-0.2)
                       ("rust-wai-bindgen-gen-rust-wasm" ,rust-wai-bindgen-gen-rust-wasm-0.2))))
    (home-page "https://wasmer.io/")
    (synopsis "Generate WAI glue for a Rust guest")
    (description "This package provides Generate WAI glue for a Rust guest.")
    (license license:asl2.0)))

(define-public rust-wai-bindgen-rust-0.2
  (package
    (name "rust-wai-bindgen-rust")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wai-bindgen-rust" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0hbpsvix4j69f8ix8jj2v8df1xydxy7ip4sy7bl67h28yk302mjf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-bitflags" ,rust-bitflags-1)
                       ("rust-wai-bindgen-rust-impl" ,rust-wai-bindgen-rust-impl-0.2))))
    (home-page "https://wasmer.io/")
    (synopsis "Generate WAI glue for a Rust guest")
    (description "This package provides Generate WAI glue for a Rust guest.")
    (license license:asl2.0)))

(define-public rust-wai-bindgen-gen-rust-wasm-0.2
  (package
    (name "rust-wai-bindgen-gen-rust-wasm")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wai-bindgen-gen-rust-wasm" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16b1i8fa1jhyaa45rdxdb4szw01zch7xayrsgz4gm1j0wzjmrwyn"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-heck" ,rust-heck-0.3)
                       ("rust-structopt" ,rust-structopt-0.3)
                       ("rust-wai-bindgen-gen-core" ,rust-wai-bindgen-gen-core-0.2)
                       ("rust-wai-bindgen-gen-rust" ,rust-wai-bindgen-gen-rust-0.2))))
    (home-page "https://wasmer.io/")
    (synopsis "Generate WAI glue for a Rust guest")
    (description "This package provides Generate WAI glue for a Rust guest.")
    (license license:asl2.0)))

(define-public rust-wasmer-wasix-types-0.4
  (package
    (name "rust-wasmer-wasix-types")
    (version "0.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-wasix-types" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "09zwm9i7qwfx7fcrv8s4agv8wqlkgsymhahzfydghabdf33aljm3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-bitflags" ,rust-bitflags-1)
                       ("rust-byteorder" ,rust-byteorder-1)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-num-enum" ,rust-num-enum-0.5)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-time" ,rust-time-0.2)
                       ("rust-wai-bindgen-gen-core" ,rust-wai-bindgen-gen-core-0.2)
                       ("rust-wai-bindgen-gen-rust" ,rust-wai-bindgen-gen-rust-0.2)
                       ("rust-wai-bindgen-gen-rust-wasm" ,rust-wai-bindgen-gen-rust-wasm-0.2)
                       ("rust-wai-bindgen-rust" ,rust-wai-bindgen-rust-0.2)
                       ("rust-wai-parser" ,rust-wai-parser-0.2)
                       ("rust-wasmer" ,rust-wasmer-3)
                       ("rust-wasmer-derive" ,rust-wasmer-derive-3)
                       ("rust-wasmer-types" ,rust-wasmer-types-3))))
    (home-page "https://wasmer.io/")
    (synopsis "WASI and WASIX types for Wasmer WebAssembly runtime")
    (description
     "This package provides WASI and WASIX types for Wasmer @code{WebAssembly} runtime.")
    (license license:expat)))

(define-public rust-wasmer-emscripten-3
  (package
    (name "rust-wasmer-emscripten")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-emscripten" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0jiz8i4gsixc4lqc2bknxrrp6pdjp9r7d8camqhbbwy2aap4d9vp"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-byteorder" ,rust-byteorder-1)
                       ("rust-getrandom" ,rust-getrandom-0.2)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-time" ,rust-time-0.2)
                       ("rust-wasmer" ,rust-wasmer-3)
                       ("rust-wasmer-types" ,rust-wasmer-types-3))))
    (home-page "https://wasmer.io/")
    (synopsis
     "Emscripten implementation library for Wasmer WebAssembly runtime")
    (description
     "This package provides Emscripten implementation library for Wasmer @code{WebAssembly} runtime.")
    (license license:expat)))

(define-public rust-wai-bindgen-gen-rust-0.2
  (package
    (name "rust-wai-bindgen-gen-rust")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wai-bindgen-gen-rust" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1zifa8yp0h46vqha6y5i2l9kjgi3zyr07vs0ghrw858573l0bg0r"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-heck" ,rust-heck-0.3)
                       ("rust-wai-bindgen-gen-core" ,rust-wai-bindgen-gen-core-0.2))))
    (home-page "https://wasmer.io/")
    (synopsis "Abstractions for generating Rust glue code for WAI")
    (description
     "This package provides Abstractions for generating Rust glue code for WAI.")
    (license license:asl2.0)))

(define-public rust-wai-bindgen-gen-wasmer-0.2
  (package
    (name "rust-wai-bindgen-gen-wasmer")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wai-bindgen-gen-wasmer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0cilwnv1kpjj1bib4x4p5xy62iplcn17yzssdnl21ifqhm0lhq8g"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-heck" ,rust-heck-0.3)
                       ("rust-structopt" ,rust-structopt-0.3)
                       ("rust-wai-bindgen-gen-core" ,rust-wai-bindgen-gen-core-0.2)
                       ("rust-wai-bindgen-gen-rust" ,rust-wai-bindgen-gen-rust-0.2))))
    (home-page "https://wasmer.io/")
    (synopsis "Generate WAI glue code for a Rust Wasmer host")
    (description
     "This package provides Generate WAI glue code for a Rust Wasmer host.")
    (license license:asl2.0)))

(define-public rust-wast-33
  (package
    (name "rust-wast")
    (version "33.0.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wast" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "06qwh49bqkalljihc8h96vkh8f37fn47sgi9f54j2y3zbhbzw10x"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-leb128" ,rust-leb128-0.2))))
    (home-page
     "https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wast")
    (synopsis
     "Customizable Rust parsers for the WebAssembly Text formats WAT and WAST")
    (description
     "This package provides Customizable Rust parsers for the @code{WebAssembly} Text formats WAT and WAST.")
    (license (list license:asl2.0))))

(define-public rust-wai-parser-0.2
  (package
    (name "rust-wai-parser")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wai-parser" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "11l1wavlk6xndpk441fkm5j5y0aglydh2j9p6h6fm188syvarl4v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-id-arena" ,rust-id-arena-2)
                       ("rust-pulldown-cmark" ,rust-pulldown-cmark-0.8)
                       ("rust-unicode-normalization" ,rust-unicode-normalization-0.1)
                       ("rust-unicode-xid" ,rust-unicode-xid-0.2)
                       ("rust-wast" ,rust-wast-33))))
    (home-page "https://wasmer.io/")
    (synopsis "Parser for WAI syntax")
    (description "This package provides Parser for WAI syntax.")
    (license license:asl2.0)))

(define-public rust-wai-bindgen-gen-core-0.2
  (package
    (name "rust-wai-bindgen-gen-core")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wai-bindgen-gen-core" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0v470cl1xqfqdlq3dy9ysv58zq179hirg088nci1308hnm0xr8qs"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-wai-parser" ,rust-wai-parser-0.2))))
    (home-page "https://wasmer.io/")
    (synopsis "Core abstractions for wai-bindgen")
    (description "This package provides Core abstractions for wai-bindgen.")
    (license license:asl2.0)))

(define-public rust-wai-bindgen-wasmer-impl-0.2
  (package
    (name "rust-wai-bindgen-wasmer-impl")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wai-bindgen-wasmer-impl" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0fl6vk406j22lbfsa8niwnm33jy6v8klxbavz0xhxpfli3nqhd2b"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-syn" ,rust-syn-1)
                       ("rust-wai-bindgen-gen-core" ,rust-wai-bindgen-gen-core-0.2)
                       ("rust-wai-bindgen-gen-wasmer" ,rust-wai-bindgen-gen-wasmer-0.2))))
    (home-page "https://wasmer.io/")
    (synopsis "Generate WAI glue for a Rust Wasmer host")
    (description
     "This package provides Generate WAI glue for a Rust Wasmer host.")
    (license license:asl2.0)))

(define-public rust-wai-bindgen-wasmer-0.4
  (package
    (name "rust-wai-bindgen-wasmer")
    (version "0.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wai-bindgen-wasmer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "015fpxp75689p9xkprw9kds79jmqnf13vg7a62vg261vdzd0w7if"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-bitflags" ,rust-bitflags-1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-wai-bindgen-wasmer-impl" ,rust-wai-bindgen-wasmer-impl-0.2)
                       ("rust-wasmer" ,rust-wasmer-3))))
    (home-page "https://wasmer.io/")
    (synopsis "Generate WAI glue for a Rust Wasmer host")
    (description
     "This package provides Generate WAI glue for a Rust Wasmer host.")
    (license license:expat)))

(define-public rust-virtual-net-0.1
  (package
    (name "rust-virtual-net")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "virtual-net" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1mzd7wigdqliz2w8z09ki2na3a11z4yz3b02sr2k8qrm7f0ynhz0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://wasmer.io/")
    (synopsis "Wasmer Virtual Networking")
    (description "This package provides Wasmer Virtual Networking.")
    (license license:expat)))

(define-public rust-wasmer-toml-0.9
  (package
    (name "rust-wasmer-toml")
    (version "0.9.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-toml" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lncanq6sxnrl1k64mbl467fa3wgzhbjnlijr8sk4i799sap456j"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-derive-builder" ,rust-derive-builder-0.12)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-semver" ,rust-semver-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-cbor" ,rust-serde-cbor-0.11)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serde-yaml" ,rust-serde-yaml-0.9)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-toml" ,rust-toml-0.8))))
    (home-page "https://wasmer.io/")
    (synopsis "parser for the wasmer.toml format used by Wasmer")
    (description
     "This package provides a parser for the wasmer.toml format used by Wasmer.")
    (license license:expat)))

(define-public rust-memmap2-0.6
  (package
    (name "rust-memmap2")
    (version "0.6.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "memmap2" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0wm8avdjma6j3x5fjdqwxcj89h52pzmwanw46xkn9rnz9albna3d"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-libc" ,rust-libc-0.2)
                       ("rust-stable-deref-trait" ,rust-stable-deref-trait-1))))
    (home-page "https://github.com/RazrFalcon/memmap2-rs")
    (synopsis "Cross-platform Rust API for memory-mapped file IO")
    (description
     "This package provides Cross-platform Rust API for memory-mapped file IO.")
    (license (list license:expat license:asl2.0))))

(define-public rust-shared-buffer-0.1
  (package
    (name "rust-shared-buffer")
    (version "0.1.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "shared-buffer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "05jbp0v6yq3s25ykf9i0r7wfbh8pxmskj794mbkmfafmp8srijgn"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytes" ,rust-bytes-1)
                       ("rust-memmap2" ,rust-memmap2-0.6))))
    (home-page "https://github.com/wasmerio/shared-buffer.git")
    (synopsis
     "An abstraction over buffers backed by memory-mapped files or bytes in memory")
    (description
     "This package provides An abstraction over buffers backed by memory-mapped files or bytes in memory.")
    (license (list license:expat license:asl2.0))))

(define-public rust-path-clean-1
  (package
    (name "rust-path-clean")
    (version "1.0.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "path-clean" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1vzwcrlz39rd94l89rppvkbsn7dvng449f1bnkyk3ayp43y9ld8p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/danreeves/path-clean")
    (synopsis "Rust implementation of cleanname or path.Clean")
    (description
     "This package provides a Rust implementation of cleanname or path.Clean.")
    (license (list license:expat license:asl2.0))))

(define-public rust-any-ascii-0.1
  (package
    (name "rust-any-ascii")
    (version "0.1.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "any_ascii" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "07kyb9is518jr1sbc6804kgg5pnx8djl328q3al28lcbxdvkf0vh"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://anyascii.com")
    (synopsis "Unicode to ASCII transliteration")
    (description "This package provides Unicode to ASCII transliteration.")
    (license license:isc)))

(define-public rust-lexical-sort-0.3
  (package
    (name "rust-lexical-sort")
    (version "0.3.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "lexical-sort" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0yi0jzlvjaszwl5a49r0a0gcq404rdk5ls2c9npis8qyc68lb7n0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-any-ascii" ,rust-any-ascii-0.1))))
    (home-page "https://lib.rs/crates/lexical-sort")
    (synopsis "Sort Unicode strings lexically")
    (description "This package provides Sort Unicode strings lexically.")
    (license (list license:expat license:asl2.0))))

(define-public rust-webc-5
  (package
    (name "rust-webc")
    (version "5.8.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "webc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "10w4334rvrxpznz6l9b54zzhlk9nz4zf1syg6yxy9csg3flsag4p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-base64" ,rust-base64-0.21)
                       ("rust-byteorder" ,rust-byteorder-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-flate2" ,rust-flate2-1)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-leb128" ,rust-leb128-0.2)
                       ("rust-lexical-sort" ,rust-lexical-sort-0.3)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-path-clean" ,rust-path-clean-1)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-semver" ,rust-semver-1)
                       ("rust-sequoia-openpgp" ,rust-sequoia-openpgp-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-cbor" ,rust-serde-cbor-0.11)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-shared-buffer" ,rust-shared-buffer-0.1)
                       ("rust-tar" ,rust-tar-0.4)
                       ("rust-tempfile" ,rust-tempfile-3)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-toml" ,rust-toml-0.7)
                       ("rust-url" ,rust-url-2)
                       ("rust-walkdir" ,rust-walkdir-2)
                       ("rust-wasmer-toml" ,rust-wasmer-toml-0.9))))
    (home-page "https://wasmer.io")
    (synopsis "WebContainer implementation for wapm.io")
    (description
     "This package provides @code{WebContainer} implementation for wapm.io.")
    (license license:expat)))

(define-public rust-virtual-fs-0.2
  (package
    (name "rust-virtual-fs")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "virtual-fs" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "13ifg7g3aj82bcgakkqaid8gy80686q6acax3fhwaxxmhrcb98hv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-derivative" ,rust-derivative-2)
                       ("rust-filetime" ,rust-filetime-0.2)
                       ("rust-fs-extra" ,rust-fs-extra-1)
                       ("rust-getrandom" ,rust-getrandom-0.2)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-pin-project-lite" ,rust-pin-project-lite-0.2)
                       ("rust-slab" ,rust-slab-0.4)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-typetag" ,rust-typetag-0.1)
                       ("rust-webc" ,rust-webc-5))))
    (home-page "https://wasmer.io/")
    (synopsis "Wasmer Virtual FileSystem")
    (description "This package provides Wasmer Virtual @code{FileSystem}.")
    (license license:expat)))

(define-public rust-typetag-impl-0.1
  (package
    (name "rust-typetag-impl")
    (version "0.1.8")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typetag-impl" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03lw15ad39bgr4m6fmr5b9lb4wapkcfsnfxsbz0362635iw4f0g6"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/dtolnay/typetag")
    (synopsis "Implementation detail of the typetag crate")
    (description
     "This package provides Implementation detail of the typetag crate.")
    (license (list license:expat license:asl2.0))))

(define-public rust-typetag-0.1
  (package
    (name "rust-typetag")
    (version "0.1.8")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typetag" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "12jva00k063gb48bvx0p0ixwbq1l48411disynzvah92bd65d020"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-erased-serde" ,rust-erased-serde-0.3)
                       ("rust-inventory" ,rust-inventory-0.2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-typetag-impl" ,rust-typetag-impl-0.1))))
    (home-page "https://github.com/dtolnay/typetag")
    (synopsis "Serde serializable and deserializable trait objects")
    (description
     "This package provides Serde serializable and deserializable trait objects.")
    (license (list license:expat license:asl2.0))))

(define-public rust-linked-hash-set-0.1
  (package
    (name "rust-linked-hash-set")
    (version "0.1.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "linked_hash_set" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "124m7wiz9ah7ah58ckai413mzfglh3y1nz64qy1s676qlinnq627"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-linked-hash-map" ,rust-linked-hash-map-0.5)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/alexheretic/linked-hash-set")
    (synopsis "HashSet with insertion ordering")
    (description
     "This package provides @code{HashSet} with insertion ordering.")
    (license license:asl2.0)))

(define-public rust-cooked-waker-5
  (package
    (name "rust-cooked-waker")
    (version "5.0.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cooked-waker" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0vs07c0am50gxzxz593sqb42jk7xs1fjs992dfydllkhcxfyayql"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/Lucretiel/cooked-waker")
    (synopsis "safe interface for creating async Wakers")
    (description
     "This package provides a safe interface for creating async Wakers.")
    (license license:mpl2.0)))

(define-public rust-wasmer-wasix-0.4
  (package
    (name "rust-wasmer-wasix")
    (version "0.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-wasix" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1g3c61601m7cmwl6w5l6sflmlxkhn8kql0gj71z2bbhsdb5zl5n2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-bincode" ,rust-bincode-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-cooked-waker" ,rust-cooked-waker-5)
                       ("rust-derivative" ,rust-derivative-2)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-getrandom" ,rust-getrandom-0.2)
                       ("rust-heapless" ,rust-heapless-0.7)
                       ("rust-hex" ,rust-hex-0.4)
                       ("rust-http" ,rust-http-0.2)
                       ("rust-hyper" ,rust-hyper-0.14)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-linked-hash-set" ,rust-linked-hash-set-0.1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-pin-project" ,rust-pin-project-1)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-reqwest" ,rust-reqwest-0.11)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-cbor" ,rust-serde-cbor-0.11)
                       ("rust-serde-derive" ,rust-serde-derive-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serde-yaml" ,rust-serde-yaml-0.8)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-shellexpand" ,rust-shellexpand-2)
                       ("rust-term-size" ,rust-term-size-0.3)
                       ("rust-termios" ,rust-termios-0.3)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tower" ,rust-tower-0.4)
                       ("rust-tower-http" ,rust-tower-http-0.4)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-typetag" ,rust-typetag-0.1)
                       ("rust-urlencoding" ,rust-urlencoding-2)
                       ("rust-virtual-fs" ,rust-virtual-fs-0.2)
                       ("rust-virtual-net" ,rust-virtual-net-0.1)
                       ("rust-wai-bindgen-wasmer" ,rust-wai-bindgen-wasmer-0.4)
                       ("rust-waker-fn" ,rust-waker-fn-1)
                       ("rust-wasm-bindgen" ,rust-wasm-bindgen-0.2)
                       ("rust-wasmer" ,rust-wasmer-3)
                       ("rust-wasmer-emscripten" ,rust-wasmer-emscripten-3)
                       ("rust-wasmer-types" ,rust-wasmer-types-3)
                       ("rust-wasmer-wasix-types" ,rust-wasmer-wasix-types-0.4)
                       ("rust-wcgi" ,rust-wcgi-0.1)
                       ("rust-wcgi-host" ,rust-wcgi-host-0.1)
                       ("rust-webc" ,rust-webc-5)
                       ("rust-weezl" ,rust-weezl-0.1)
                       ("rust-winapi" ,rust-winapi-0.3))))
    (home-page "https://wasmer.io/")
    (synopsis
     "WASI and WASIX implementation library for Wasmer WebAssembly runtime")
    (description
     "This package provides WASI and WASIX implementation library for Wasmer @code{WebAssembly} runtime.")
    (license license:expat)))

(define-public rust-wasmer-cache-3
  (package
    (name "rust-wasmer-cache")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-cache" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "070jxk15hwiz26afwxyphyhi4dwahvjldsxyjq8w3hswn1lyj3bz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-blake3" ,rust-blake3-1)
                       ("rust-hex" ,rust-hex-0.4)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-wasmer" ,rust-wasmer-3))))
    (home-page "https://wasmer.io/")
    (synopsis "Cache system for Wasmer WebAssembly runtime")
    (description
     "This package provides Cache system for Wasmer @code{WebAssembly} runtime.")
    (license license:expat)))

(define-public rust-wasmparser-0.214
  (package
    (name "rust-wasmparser")
    (version "0.214.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmparser" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1dnw7nkvxsph7718qikyp3nxlgwkx5j21x42sg8dm11y1q4w22ak"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-hashbrown" ,rust-hashbrown-0.14)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-semver" ,rust-semver-1)
                       ("rust-serde" ,rust-serde-1))))
    (home-page
     "https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasmparser")
    (synopsis
     "simple event-driven library for parsing WebAssembly binary files.")
    (description
     "This package provides a simple event-driven library for parsing
@code{WebAssembly} binary files.")
    (license (list license:asl2.0))))

(define-public rust-wasm-encoder-0.214
  (package
    (name "rust-wasm-encoder")
    (version "0.214.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasm-encoder" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ha0yksmd10lnvkvx7xfnb6iigrzi01sx5xi49lhp9fpm014ysgz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-leb128" ,rust-leb128-0.2)
                       ("rust-wasmparser" ,rust-wasmparser-0.214))))
    (home-page
     "https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-encoder")
    (synopsis "low-level WebAssembly encoder.")
    (description
     "This package provides a low-level @code{WebAssembly} encoder.")
    (license (list license:asl2.0))))

(define-public rust-fallible-iterator-0.3
  (package
    (name "rust-fallible-iterator")
    (version "0.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "fallible-iterator" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ja6l56yka5vn4y4pk6hn88z0bpny7a8k1919aqjzp0j1yhy9k1a"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/sfackler/rust-fallible-iterator")
    (synopsis "Fallible iterator traits")
    (description "This package provides Fallible iterator traits.")
    (license (list license:expat license:asl2.0))))

(define-public rust-gimli-0.30
  (package
    (name "rust-gimli")
    (version "0.30.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "gimli" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jfz1sj9h0rnlhjnmy315db2drsn54f0b5qcpndvl8lpprzxkqg2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-compiler-builtins" ,rust-compiler-builtins-0.1)
                       ("rust-fallible-iterator" ,rust-fallible-iterator-0.3)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-rustc-std-workspace-alloc" ,rust-rustc-std-workspace-alloc-1)
                       ("rust-rustc-std-workspace-core" ,rust-rustc-std-workspace-core-1)
                       ("rust-stable-deref-trait" ,rust-stable-deref-trait-1))))
    (home-page "https://github.com/gimli-rs/gimli")
    (synopsis "library for reading and writing the DWARF debugging format.")
    (description
     "This package provides a library for reading and writing the DWARF debugging
format.")
    (license (list license:expat license:asl2.0))))

(define-public rust-bumpalo-3
  (package
    (name "rust-bumpalo")
    (version "3.16.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "bumpalo" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0b015qb4knwanbdlp1x48pkb4pm57b8gidbhhhxr900q2wb6fabr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-allocator-api2" ,rust-allocator-api2-0.2)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/fitzgen/bumpalo")
    (synopsis "fast bump allocation arena for Rust.")
    (description
     "This package provides a fast bump allocation arena for Rust.")
    (license (list license:expat license:asl2.0))))

(define-public rust-wast-214
  (package
    (name "rust-wast")
    (version "214.0.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wast" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1sgqlz8diw9hsmay7vi37611x8812fvnhdviv2dp1j299jrcsjv9"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bumpalo" ,rust-bumpalo-3)
                       ("rust-gimli" ,rust-gimli-0.30)
                       ("rust-leb128" ,rust-leb128-0.2)
                       ("rust-memchr" ,rust-memchr-2)
                       ("rust-unicode-width" ,rust-unicode-width-0.1)
                       ("rust-wasm-encoder" ,rust-wasm-encoder-0.214))))
    (home-page
     "https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wast")
    (synopsis
     "Customizable Rust parsers for the WebAssembly Text formats WAT and WAST")
    (description
     "This package provides Customizable Rust parsers for the @code{WebAssembly} Text formats WAT and WAST.")
    (license (list license:asl2.0))))

(define-public rust-wat-1
  (package
    (name "rust-wat")
    (version "1.214.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wat" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "13i71rwxjf88bq5jm4hssvg3g561m3iwymi6vwlafgvpavmljwil"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-wast" ,rust-wast-214))))
    (home-page
     "https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wat")
    (synopsis "Rust parser for the WebAssembly Text format, WAT")
    (description
     "This package provides Rust parser for the @code{WebAssembly} Text format, WAT.")
    (license (list license:asl2.0))))

(define-public rust-wasmparser-0.83
  (package
    (name "rust-wasmparser")
    (version "0.83.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmparser" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0yhx2kq7da4sdglh1x1di4xxg33k7lwddpd3ri46bp9abk2xg3ki"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page
     "https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasmparser")
    (synopsis
     "simple event-driven library for parsing WebAssembly binary files.")
    (description
     "This package provides a simple event-driven library for parsing
@code{WebAssembly} binary files.")
    (license (list license:asl2.0))))

(define-public rust-wasmer-derive-3
  (package
    (name "rust-wasmer-derive")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0f3p63wvx6qb9j5r9ixf76n5iym9fqxcqqm11swxp0z3mbdiz44p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro-error" ,rust-proc-macro-error-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://wasmer.io/")
    (synopsis "Wasmer derive macros")
    (description "This package provides Wasmer derive macros.")
    (license license:expat)))

(define-public rust-dynasmrt-1
  (package
    (name "rust-dynasmrt")
    (version "1.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "dynasmrt" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1aa1av01h0l8ms9fk32ydahby77fd3hhv85zsk51fsnp5fjabyv4"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-byteorder" ,rust-byteorder-1)
                       ("rust-dynasm" ,rust-dynasm-1)
                       ("rust-memmap2" ,rust-memmap2-0.5))))
    (home-page "https://github.com/CensoredUsername/dynasm-rs")
    (synopsis
     "simple runtime for assembling code at runtime. Combined with the plugin crate dynasm it can be used to write JIT compilers easily.")
    (description
     "This package provides a simple runtime for assembling code at runtime.  Combined
with the plugin crate dynasm it can be used to write JIT compilers easily.")
    (license license:mpl2.0)))

(define-public rust-dynasm-1
  (package
    (name "rust-dynasm")
    (version "1.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "dynasm" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "12yf5l193j318bv9fxqw1r5j210mdzh0jgrna304wlkvh01a3ndd"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-1)
                       ("rust-byteorder" ,rust-byteorder-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-proc-macro-error" ,rust-proc-macro-error-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/CensoredUsername/dynasm-rs")
    (synopsis
     "plugin for assembling code at runtime. Combined with the runtime crate dynasmrt it can be used to write JIT compilers easily.")
    (description
     "This package provides a plugin for assembling code at runtime.  Combined with
the runtime crate dynasmrt it can be used to write JIT compilers easily.")
    (license license:mpl2.0)))

(define-public rust-wasmer-compiler-singlepass-3
  (package
    (name "rust-wasmer-compiler-singlepass")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-compiler-singlepass" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "15ybads0p9kxpx5lvdb5sr4f53i6xj6g0gxgq0aj2ibgvrbqklyl"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-byteorder" ,rust-byteorder-1)
                       ("rust-dynasm" ,rust-dynasm-1)
                       ("rust-dynasmrt" ,rust-dynasmrt-1)
                       ("rust-enumset" ,rust-enumset-1)
                       ("rust-gimli" ,rust-gimli-0.26)
                       ("rust-hashbrown" ,rust-hashbrown-0.11)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-more-asserts" ,rust-more-asserts-0.2)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-wasmer-compiler" ,rust-wasmer-compiler-3)
                       ("rust-wasmer-types" ,rust-wasmer-types-3))))
    (home-page "https://wasmer.io/")
    (synopsis "Singlepass compiler for Wasmer WebAssembly runtime")
    (description
     "This package provides Singlepass compiler for Wasmer @code{WebAssembly} runtime.")
    (license license:expat)))

(define-public rust-alloc-traits-0.1
  (package
    (name "rust-alloc-traits")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "alloc-traits" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "10z4rmykwnp8ps5r5n34190h6gmzpj1f67fqildi1z8r6f2m8bbb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/HeroicKatora/static-alloc")
    (synopsis "Traits to replace or supplement the alloc module in no_std")
    (description
     "This package provides Traits to replace or supplement the alloc module in no_std.")
    (license license:expat)))

(define-public rust-static-alloc-0.2
  (package
    (name "rust-static-alloc")
    (version "0.2.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "static-alloc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "07p6s9njqc1v6jpr0vlw55ps4v32wp3df27fxjg565nf6ph7aacb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-alloc-traits" ,rust-alloc-traits-0.1)
                       ("rust-atomic-polyfill" ,rust-atomic-polyfill-1))))
    (home-page "https://github.com/HeroicKatora/static-alloc")
    (synopsis "bump allocator on static memory for the alloc-traits crate")
    (description
     "This package provides a bump allocator on static memory for the alloc-traits
crate.")
    (license (list license:expat license:asl2.0 license:zlib))))

(define-public rust-llvm-sys-130
  (package
    (name "rust-llvm-sys")
    (version "130.1.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0z6nzm49gav2x2mvmvqjsqqphrkr5w8hvbyvdq1y9gf43z4l40f2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-clang-sys" ,rust-clang-sys-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-semver" ,rust-semver-0.11)
                       ("rust-tempfile" ,rust-tempfile-3))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-llvm-sys-110
  (package
    (name "rust-llvm-sys")
    (version "110.0.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0gbwa9i39l0rx66v8wgrg6ixpf7r35vvj9kxplg98q29qksddfks"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-semver" ,rust-semver-0.11))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-llvm-sys-120
  (package
    (name "rust-llvm-sys")
    (version "120.3.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1vr1kvqr0z2pr4x4gl1b9dd71fbg7z5fw4ssp1z9qxinyj92ckv2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-semver" ,rust-semver-0.11))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-llvm-sys-90
  (package
    (name "rust-llvm-sys")
    (version "90.2.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1fbrfchskrbgvf6ri2sg1qf05w53a093kig7g0vnfz3yggav0dhh"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-semver" ,rust-semver-0.9))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-llvm-sys-80
  (package
    (name "rust-llvm-sys")
    (version "80.3.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "06nlp13zc47q55zscqz6fdhvh4cxjpdcxs7q23r5pz6dzpxpd9aw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-semver" ,rust-semver-0.9))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-llvm-sys-70
  (package
    (name "rust-llvm-sys")
    (version "70.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1k9z7qdrss9dfdc64bbbq846plvgp98yir7yyzk7hy61gq900cv7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-semver" ,rust-semver-0.9))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-llvm-sys-60
  (package
    (name "rust-llvm-sys")
    (version "60.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ky5xqmc5k7lq9lwfp95yinyxzwsd3d5x06gybcl0scvd4wsg9z8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-0.2)
                       ("rust-semver" ,rust-semver-0.9))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-llvm-sys-50
  (package
    (name "rust-llvm-sys")
    (version "50.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "15g89sslrf5c6z5bdrpssimpsr2il9bwaaql5nvscgwybzrhizdf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-lazy-static" ,rust-lazy-static-0.2)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-0.2)
                       ("rust-semver" ,rust-semver-0.6))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-semver-0.6
  (package
    (name "rust-semver")
    (version "0.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "semver" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0drmjiwkq0znj33q9x9hw7ld8f28n9diyjs3jlh1l1v5kvn8ccbs"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-semver-parser" ,rust-semver-parser-0.7))))
    (home-page "https://github.com/dtolnay/semver")
    (synopsis "Parser and evaluator for Cargo's flavor of Semantic Versioning")
    (description
     "This package provides Parser and evaluator for Cargo's flavor of Semantic Versioning.")
    (license (list license:expat license:asl2.0))))

(define-public rust-llvm-sys-40
  (package
    (name "rust-llvm-sys")
    (version "40.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "099lk8w19is1s4hywilpmnaqp5lm8gp2hw5f06g6f8xn65yi7sm7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-lazy-static" ,rust-lazy-static-0.2)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-0.2)
                       ("rust-semver" ,rust-semver-0.6))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-llvm-sys-150
  (package
    (name "rust-llvm-sys")
    (version "150.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0vdji66q9azps2k14jy4p204hg0bvghxysgp8ki8n2p9zcd8kml8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-clang-sys" ,rust-clang-sys-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-semver" ,rust-semver-1)
                       ("rust-tempfile" ,rust-tempfile-3))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-clang-sys-1
  (package
    (name "rust-clang-sys")
    (version "1.8.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "clang-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1x1r9yqss76z8xwpdanw313ss6fniwc1r7dzb5ycjn0ph53kj0hb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-glob" ,rust-glob-0.3)
                       ("rust-glob" ,rust-glob-0.3)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-libloading" ,rust-libloading-0.8))))
    (home-page "https://github.com/KyleMayes/clang-sys")
    (synopsis "Rust bindings for libclang")
    (description "This package provides Rust bindings for libclang.")
    (license license:asl2.0)))

(define-public rust-llvm-sys-140
  (package
    (name "rust-llvm-sys")
    (version "140.1.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "030srgq5vgshgpmb0w05gpf3yjbh73vcrgg327n320kwhplpip73"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-clang-sys" ,rust-clang-sys-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-semver" ,rust-semver-1)
                       ("rust-tempfile" ,rust-tempfile-3))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-llvm-sys-100
  (package
    (name "rust-llvm-sys")
    (version "100.2.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "llvm-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0d2v8y9fwqqhidmdxhra7509vj503bj0zlivp9wm1xw4g28il9bj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-semver" ,rust-semver-0.9))))
    (home-page "https://gitlab.com/taricorp/llvm-sys.rs")
    (synopsis "Bindings to LLVM's C API")
    (description "This package provides Bindings to LLVM's C API.")
    (license license:expat)))

(define-public rust-inkwell-internals-0.7
  (package
    (name "rust-inkwell-internals")
    (version "0.7.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "inkwell_internals" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "11gah33af1bbidmx1pdy6xqyr24ijpzpngf2bfz05kk4w8bhrl47"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/TheDan64/inkwell")
    (synopsis "Internal macro crate for inkwell")
    (description "This package provides Internal macro crate for inkwell.")
    (license license:asl2.0)))

(define-public rust-inkwell-0.1
  (package
    (name "rust-inkwell")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "inkwell" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jfyqsq4fc5blkkgs9gnf9m3wic136k6lzpvcxc556hmhpj13b5v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-either" ,rust-either-1)
                       ("rust-inkwell-internals" ,rust-inkwell-internals-0.7)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-llvm-sys" ,rust-llvm-sys-100)
                       ("rust-llvm-sys" ,rust-llvm-sys-140)
                       ("rust-llvm-sys" ,rust-llvm-sys-150)
                       ("rust-llvm-sys" ,rust-llvm-sys-40)
                       ("rust-llvm-sys" ,rust-llvm-sys-50)
                       ("rust-llvm-sys" ,rust-llvm-sys-60)
                       ("rust-llvm-sys" ,rust-llvm-sys-70)
                       ("rust-llvm-sys" ,rust-llvm-sys-80)
                       ("rust-llvm-sys" ,rust-llvm-sys-90)
                       ("rust-llvm-sys" ,rust-llvm-sys-120)
                       ("rust-llvm-sys" ,rust-llvm-sys-110)
                       ("rust-llvm-sys" ,rust-llvm-sys-130)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-static-alloc" ,rust-static-alloc-0.2))))
    (home-page "https://github.com/TheDan64/inkwell")
    (synopsis
     "Inkwell aims to help you pen your own programming languages by safely wrapping llvm-sys")
    (description
     "This package provides Inkwell aims to help you pen your own programming languages by safely wrapping
llvm-sys.")
    (license license:asl2.0)))

(define-public rust-wasmer-compiler-llvm-3
  (package
    (name "rust-wasmer-compiler-llvm")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-compiler-llvm" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1khs46ipa95xjz7rcwn23ggj39a8i2g1pgywr1bw5zb442ddhv6v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-byteorder" ,rust-byteorder-1)
                       ("rust-cc" ,rust-cc-1)
                       ("rust-inkwell" ,rust-inkwell-0.1)
                       ("rust-itertools" ,rust-itertools-0.10)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-object" ,rust-object-0.28)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-rustc-version" ,rust-rustc-version-0.4)
                       ("rust-semver" ,rust-semver-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-target-lexicon" ,rust-target-lexicon-0.12)
                       ("rust-wasmer-compiler" ,rust-wasmer-compiler-3)
                       ("rust-wasmer-types" ,rust-wasmer-types-3)
                       ("rust-wasmer-vm" ,rust-wasmer-vm-3))))
    (home-page "https://wasmer.io/")
    (synopsis "LLVM compiler for Wasmer WebAssembly runtime")
    (description
     "This package provides LLVM compiler for Wasmer @code{WebAssembly} runtime.")
    (license license:expat)))

(define-public rust-cranelift-frontend-0.91
  (package
    (name "rust-cranelift-frontend")
    (version "0.91.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cranelift-frontend" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "19mc2ls24c2yrnj29j6svxkpj3ypq7lkd23yzz43vvygp2nanw0d"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cranelift-codegen" ,rust-cranelift-codegen-0.91)
                       ("rust-hashbrown" ,rust-hashbrown-0.12)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-target-lexicon" ,rust-target-lexicon-0.12))))
    (home-page "https://github.com/bytecodealliance/wasmtime")
    (synopsis "Cranelift IR builder helper")
    (description "This package provides Cranelift IR builder helper.")
    (license (list license:asl2.0))))

(define-public rust-id-arena-2
  (package
    (name "rust-id-arena")
    (version "2.2.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "id-arena" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "01ch8jhpgnih8sawqs44fqsqpc7bzwgy0xpi6j0f4j0i5mkvr8i5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-rayon" ,rust-rayon-1))))
    (home-page "https://github.com/fitzgen/id-arena")
    (synopsis "simple, id-based arena.")
    (description "This package provides a simple, id-based arena.")
    (license (list license:expat license:asl2.0))))

(define-public rust-souper-ir-2
  (package
    (name "rust-souper-ir")
    (version "2.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "souper-ir" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0i60q84w5k3rd0j3zhsdc5xasrd4wrkamyrs01rik3lq6g71h355"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-id-arena" ,rust-id-arena-2))))
    (home-page "https://github.com/fitzgen/souper-ir")
    (synopsis "library for manipulating Souper IR")
    (description "This package provides a library for manipulating Souper IR.")
    (license (list license:expat license:asl2.0))))

(define-public rust-slice-group-by-0.3
  (package
    (name "rust-slice-group-by")
    (version "0.3.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "slice-group-by" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "19vbyyxqvc25fv2dmhlxijlk5sa9j34yb6hyydb9vf89kh36fqc2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/Kerollmops/slice-group-by")
    (synopsis "Iterators over groups in slices and strs")
    (description
     "This package provides Iterators over groups in slices and strs.")
    (license license:expat)))

(define-public rust-regalloc2-0.5
  (package
    (name "rust-regalloc2")
    (version "0.5.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "regalloc2" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0303kga9n8695w7pvf4p5zspq90h24fd9pd3ifknc70cnjzly39h"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-fxhash" ,rust-fxhash-0.2)
                       ("rust-libfuzzer-sys" ,rust-libfuzzer-sys-0.4)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-slice-group-by" ,rust-slice-group-by-0.3)
                       ("rust-smallvec" ,rust-smallvec-1))))
    (home-page "https://github.com/bytecodealliance/regalloc2")
    (synopsis "Backtracking register allocator inspired from IonMonkey")
    (description
     "This package provides Backtracking register allocator inspired from @code{IonMonkey}.")
    (license (list license:asl2.0))))

(define-public rust-cranelift-isle-0.91
  (package
    (name "rust-cranelift-isle")
    (version "0.91.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cranelift-isle" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1fw5swg2j2vwj4cn76grhjh43jyn8ccgc1rspf6zyc0q8lycffrr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-codespan-reporting" ,rust-codespan-reporting-0.11)
                       ("rust-log" ,rust-log-0.4))))
    (home-page
     "https://github.com/bytecodealliance/wasmtime/tree/main/cranelift/isle")
    (synopsis
     "ISLE: Instruction Selection and Lowering Expressions. A domain-specific language for instruction selection in Cranelift")
    (description
     "This package provides ISLE: Instruction Selection and Lowering Expressions.  A domain-specific
language for instruction selection in Cranelift.")
    (license (list license:asl2.0))))

(define-public rust-cranelift-egraph-0.91
  (package
    (name "rust-cranelift-egraph")
    (version "0.91.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cranelift-egraph" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0wwd5j9sfpdyi4ccws046jxj7lw2p91ijc9r74lpbrh67cr58jv2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cranelift-entity" ,rust-cranelift-entity-0.91)
                       ("rust-fxhash" ,rust-fxhash-0.2)
                       ("rust-hashbrown" ,rust-hashbrown-0.12)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-smallvec" ,rust-smallvec-1))))
    (home-page "https://github.com/bytecodealliance/wasmtime")
    (synopsis "acyclic-egraph (aegraph) implementation for Cranelift")
    (description
     "This package provides acyclic-egraph (aegraph) implementation for Cranelift.")
    (license (list license:asl2.0))))

(define-public rust-cranelift-codegen-shared-0.91
  (package
    (name "rust-cranelift-codegen-shared")
    (version "0.91.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cranelift-codegen-shared" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1dws7bim1gxiqlahfnnh0q1sg6an55n4027g64jg7z2kkki553i7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/bytecodealliance/wasmtime")
    (synopsis
     "For code shared between cranelift-codegen-meta and cranelift-codegen")
    (description
     "This package provides For code shared between cranelift-codegen-meta and cranelift-codegen.")
    (license (list license:asl2.0))))

(define-public rust-cranelift-codegen-meta-0.91
  (package
    (name "rust-cranelift-codegen-meta")
    (version "0.91.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cranelift-codegen-meta" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0x3ffp0d86ggz7ggqzknrppwp1dh187hqc7qk0m13b9lajs0g4v3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cranelift-codegen-shared" ,rust-cranelift-codegen-shared-0.91))))
    (home-page "https://github.com/bytecodealliance/wasmtime")
    (synopsis "Metaprogram for cranelift-codegen code generator library")
    (description
     "This package provides Metaprogram for cranelift-codegen code generator library.")
    (license (list license:asl2.0))))

(define-public rust-cranelift-entity-0.91
  (package
    (name "rust-cranelift-entity")
    (version "0.91.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cranelift-entity" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0k4x3bi45cawab3j7nyscczby32ybsxcpcrsp5q1ngwwm2ybqncs"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/bytecodealliance/wasmtime")
    (synopsis "Data structures using entity references as mapping keys")
    (description
     "This package provides Data structures using entity references as mapping keys.")
    (license (list license:asl2.0))))

(define-public rust-cranelift-bforest-0.91
  (package
    (name "rust-cranelift-bforest")
    (version "0.91.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cranelift-bforest" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "05811vi3rp6j3678jv4yky51ms3nbwcklh44w55nyfpx5m8v8aia"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cranelift-entity" ,rust-cranelift-entity-0.91))))
    (home-page "https://github.com/bytecodealliance/wasmtime")
    (synopsis "forest of B+-trees")
    (description "This package provides a forest of B+-trees.")
    (license (list license:asl2.0))))

(define-public rust-cranelift-codegen-0.91
  (package
    (name "rust-cranelift-codegen")
    (version "0.91.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cranelift-codegen" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1p3128qg8b9n8fc0821v55hjcd1my5ngxfnz764a64sr5bnj5c4q"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-bincode" ,rust-bincode-1)
                       ("rust-bumpalo" ,rust-bumpalo-3)
                       ("rust-cranelift-bforest" ,rust-cranelift-bforest-0.91)
                       ("rust-cranelift-codegen-meta" ,rust-cranelift-codegen-meta-0.91)
                       ("rust-cranelift-codegen-shared" ,rust-cranelift-codegen-shared-0.91)
                       ("rust-cranelift-egraph" ,rust-cranelift-egraph-0.91)
                       ("rust-cranelift-entity" ,rust-cranelift-entity-0.91)
                       ("rust-cranelift-isle" ,rust-cranelift-isle-0.91)
                       ("rust-gimli" ,rust-gimli-0.26)
                       ("rust-hashbrown" ,rust-hashbrown-0.12)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-regalloc2" ,rust-regalloc2-0.5)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-souper-ir" ,rust-souper-ir-2)
                       ("rust-target-lexicon" ,rust-target-lexicon-0.12))))
    (home-page "https://github.com/bytecodealliance/wasmtime")
    (synopsis "Low-level code generator library")
    (description "This package provides Low-level code generator library.")
    (license (list license:asl2.0))))

(define-public rust-wasmer-compiler-cranelift-3
  (package
    (name "rust-wasmer-compiler-cranelift")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-compiler-cranelift" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "15bcclp2s48p82szjv75j0fa36xjqycrfgyzpcz5cwpkj0zn7rd1"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cranelift-codegen" ,rust-cranelift-codegen-0.91)
                       ("rust-cranelift-entity" ,rust-cranelift-entity-0.91)
                       ("rust-cranelift-frontend" ,rust-cranelift-frontend-0.91)
                       ("rust-gimli" ,rust-gimli-0.26)
                       ("rust-hashbrown" ,rust-hashbrown-0.11)
                       ("rust-more-asserts" ,rust-more-asserts-0.2)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-target-lexicon" ,rust-target-lexicon-0.12)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-wasmer-compiler" ,rust-wasmer-compiler-3)
                       ("rust-wasmer-types" ,rust-wasmer-types-3))))
    (home-page "https://wasmer.io/")
    (synopsis "Cranelift compiler for Wasmer WebAssembly runtime")
    (description
     "This package provides Cranelift compiler for Wasmer @code{WebAssembly} runtime.")
    (license (list license:expat license:asl2.0))))

(define-public rust-wasmparser-0.95
  (package
    (name "rust-wasmparser")
    (version "0.95.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmparser" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0yn8ma731gpzra72k0sh84v8h3db07df2js1698v36gafdi8kspj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-indexmap" ,rust-indexmap-1)
                       ("rust-url" ,rust-url-2))))
    (home-page
     "https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasmparser")
    (synopsis
     "simple event-driven library for parsing WebAssembly binary files.")
    (description
     "This package provides a simple event-driven library for parsing
@code{WebAssembly} binary files.")
    (license (list license:asl2.0))))

(define-public rust-windows-x86-64-msvc-0.33
  (package
    (name "rust-windows-x86-64-msvc")
    (version "0.33.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "windows_x86_64_msvc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1akf81g0bh8mv8wjpiifd819r0hx3r0xrz9zgzn4hl298sk4l7pz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/microsoft/windows-rs")
    (synopsis "Import lib for Windows")
    (description "This package provides Import lib for Windows.")
    (license (list license:expat license:asl2.0))))

(define-public rust-windows-x86-64-gnu-0.33
  (package
    (name "rust-windows-x86-64-gnu")
    (version "0.33.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "windows_x86_64_gnu" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1127n961nib9338n0g0sp1464v8wnw0hvmw45sr7pkly1q69ppdl"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/microsoft/windows-rs")
    (synopsis "Import lib for Windows")
    (description "This package provides Import lib for Windows.")
    (license (list license:expat license:asl2.0))))

(define-public rust-windows-i686-msvc-0.33
  (package
    (name "rust-windows-i686-msvc")
    (version "0.33.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "windows_i686_msvc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1l3kwxgdfg4lnx2j5bkcx6cnvhxnpcsbqjm3idhwxmwsrj4vxzcc"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/microsoft/windows-rs")
    (synopsis "Import lib for Windows")
    (description "This package provides Import lib for Windows.")
    (license (list license:expat license:asl2.0))))

(define-public rust-windows-i686-gnu-0.33
  (package
    (name "rust-windows-i686-gnu")
    (version "0.33.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "windows_i686_gnu" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03nsi8h0yd4n9wgpxcpynzwlnacihisgmh021kfb5fln79qczc6a"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/microsoft/windows-rs")
    (synopsis "Import lib for Windows")
    (description "This package provides Import lib for Windows.")
    (license (list license:expat license:asl2.0))))

(define-public rust-windows-aarch64-msvc-0.33
  (package
    (name "rust-windows-aarch64-msvc")
    (version "0.33.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "windows_aarch64_msvc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "01q80v2zzfc144xsqj3yhd62rn1dy1kyamhyv0gcrf4sxg9iyxnd"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/microsoft/windows-rs")
    (synopsis "Import lib for Windows")
    (description "This package provides Import lib for Windows.")
    (license (list license:expat license:asl2.0))))

(define-public rust-windows-sys-0.33
  (package
    (name "rust-windows-sys")
    (version "0.33.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "windows-sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0xdym5hgf2pp5lmfdjk4lynad99w4j02v9yzn6752a9ncsbb1ns3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-windows-aarch64-msvc" ,rust-windows-aarch64-msvc-0.33)
                       ("rust-windows-i686-gnu" ,rust-windows-i686-gnu-0.33)
                       ("rust-windows-i686-msvc" ,rust-windows-i686-msvc-0.33)
                       ("rust-windows-x86-64-gnu" ,rust-windows-x86-64-gnu-0.33)
                       ("rust-windows-x86-64-msvc" ,rust-windows-x86-64-msvc-0.33))))
    (home-page "https://github.com/microsoft/windows-rs")
    (synopsis "Rust for Windows")
    (description "This package provides Rust for Windows.")
    (license (list license:expat license:asl2.0))))

(define-public rust-corosensei-0.1
  (package
    (name "rust-corosensei")
    (version "0.1.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "corosensei" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "13ksv3m4w94xv59h8h0y5ixlh94j8kn5k9yj878cpacfqlr8h4l0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-autocfg" ,rust-autocfg-1)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-scopeguard" ,rust-scopeguard-1)
                       ("rust-windows-sys" ,rust-windows-sys-0.33))))
    (home-page "https://github.com/Amanieu/corosensei")
    (synopsis "fast and safe implementation of stackful coroutines")
    (description
     "This package provides a fast and safe implementation of stackful coroutines.")
    (license (list license:expat license:asl2.0))))

(define-public rust-wasmer-vm-3
  (package
    (name "rust-wasmer-vm")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-vm" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0gsn000jkbf94s31img1cpc4p1iba25pbarsxm7p3l8n9zn1hc84"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-backtrace" ,rust-backtrace-0.3)
                       ("rust-cc" ,rust-cc-1)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-corosensei" ,rust-corosensei-0.1)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-derivative" ,rust-derivative-2)
                       ("rust-enum-iterator" ,rust-enum-iterator-0.7)
                       ("rust-fnv" ,rust-fnv-1)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-mach" ,rust-mach-0.3)
                       ("rust-memoffset" ,rust-memoffset-0.8)
                       ("rust-more-asserts" ,rust-more-asserts-0.2)
                       ("rust-region" ,rust-region-3)
                       ("rust-scopeguard" ,rust-scopeguard-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-wasmer-types" ,rust-wasmer-types-3)
                       ("rust-winapi" ,rust-winapi-0.3))))
    (home-page "https://wasmer.io/")
    (synopsis "Runtime library support for Wasmer")
    (description "This package provides Runtime library support for Wasmer.")
    (license (list license:expat license:asl2.0))))

(define-public rust-wasmer-types-3
  (package
    (name "rust-wasmer-types")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-types" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16v5bmrrsm0mvagwfl9bhspz66cfcl7cvpg4a2am0qagky1z5wb7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytecheck" ,rust-bytecheck-0.6)
                       ("rust-enum-iterator" ,rust-enum-iterator-0.7)
                       ("rust-enumset" ,rust-enumset-1)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-more-asserts" ,rust-more-asserts-0.2)
                       ("rust-rkyv" ,rust-rkyv-0.7)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-bytes" ,rust-serde-bytes-0.11)
                       ("rust-target-lexicon" ,rust-target-lexicon-0.12)
                       ("rust-thiserror" ,rust-thiserror-1))))
    (home-page "https://wasmer.io/")
    (synopsis "Wasmer Common Types")
    (description "This package provides Wasmer Common Types.")
    (license (list license:expat license:asl2.0))))

(define-public rust-object-0.28
  (package
    (name "rust-object")
    (version "0.28.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "object" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0964501nlfh806mik3f9v6n05mx74qa0w7byvn0sqpwm5lprhb74"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-compiler-builtins" ,rust-compiler-builtins-0.1)
                       ("rust-crc32fast" ,rust-crc32fast-1)
                       ("rust-flate2" ,rust-flate2-1)
                       ("rust-hashbrown" ,rust-hashbrown-0.11)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-memchr" ,rust-memchr-2)
                       ("rust-rustc-std-workspace-alloc" ,rust-rustc-std-workspace-alloc-1)
                       ("rust-rustc-std-workspace-core" ,rust-rustc-std-workspace-core-1)
                       ("rust-wasmparser" ,rust-wasmparser-0.57))))
    (home-page "https://github.com/gimli-rs/object")
    (synopsis "unified interface for reading and writing object file formats.")
    (description
     "This package provides a unified interface for reading and writing object file
formats.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-wasmer-object-3
  (package
    (name "rust-wasmer-object")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-object" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1bja4is4whdzfrldgzksh92mxfhzmh22ikmzprrj23yx9zh2ashv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-object" ,rust-object-0.28)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-wasmer-types" ,rust-wasmer-types-3))))
    (home-page "https://wasmer.io/")
    (synopsis "Wasmer Native Object generator")
    (description "This package provides Wasmer Native Object generator.")
    (license license:expat)))

(define-public rust-leb128-0.2
  (package
    (name "rust-leb128")
    (version "0.2.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "leb128" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0rxxjdn76sjbrb08s4bi7m4x47zg68f71jzgx8ww7j0cnivjckl8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/gimli-rs/leb128")
    (synopsis
     "Read and write DWARF's \"Little Endian Base 128\" (LEB128) variable length integer encoding")
    (description
     "This package provides Read and write DWARF's \"Little Endian Base 128\" (LEB128) variable length integer
encoding.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-enum-iterator-derive-0.7
  (package
    (name "rust-enum-iterator-derive")
    (version "0.7.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "enum-iterator-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ndihb41kmi6pxc2bs097abxliw2pgnnw412lhdqfymjc1vw6d61"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/stephaneyfx/enum-iterator")
    (synopsis "Procedural macro to derive Sequence")
    (description "This package provides Procedural macro to derive Sequence.")
    (license license:bsd-0)))

(define-public rust-enum-iterator-0.7
  (package
    (name "rust-enum-iterator")
    (version "0.7.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "enum-iterator" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1rldnx6avpz39i1bwb65d4gs303p40syyfc4zqwlx7mpxp2wbsjf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-enum-iterator-derive" ,rust-enum-iterator-derive-0.7))))
    (home-page "https://github.com/stephaneyfx/enum-iterator")
    (synopsis
     "Tools to iterate over all values of a type (e.g. all variants of an enumeration)")
    (description
     "This package provides tools to iterate over all values of a type (e.g. all
variants of an enumeration).")
    (license license:bsd-0)))

(define-public rust-wasmer-compiler-3
  (package
    (name "rust-wasmer-compiler")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer-compiler" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "103id38x3pvpjm137ry7dj4rzixzzmrnjyxg6iyabx0sai888skj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-backtrace" ,rust-backtrace-0.3)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-enum-iterator" ,rust-enum-iterator-0.7)
                       ("rust-enumset" ,rust-enumset-1)
                       ("rust-hashbrown" ,rust-hashbrown-0.11)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-leb128" ,rust-leb128-0.2)
                       ("rust-memmap2" ,rust-memmap2-0.5)
                       ("rust-more-asserts" ,rust-more-asserts-0.2)
                       ("rust-region" ,rust-region-3)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-bytes" ,rust-serde-bytes-0.11)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-wasmer-object" ,rust-wasmer-object-3)
                       ("rust-wasmer-types" ,rust-wasmer-types-3)
                       ("rust-wasmer-vm" ,rust-wasmer-vm-3)
                       ("rust-wasmparser" ,rust-wasmparser-0.95)
                       ("rust-winapi" ,rust-winapi-0.3))))
    (home-page "https://wasmer.io/")
    (synopsis "Base compiler abstraction for Wasmer WebAssembly runtime")
    (description
     "This package provides Base compiler abstraction for Wasmer @code{WebAssembly} runtime.")
    (license (list license:expat license:asl2.0))))

(define-public rust-wasm-bindgen-downcast-macros-0.1
  (package
    (name "rust-wasm-bindgen-downcast-macros")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasm-bindgen-downcast-macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "13zk8l5qligf53ah2my2gqn15z61wd25s18qy7zcxkn7hzx0q0n5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://wasmer.io/")
    (synopsis "The proc-macro crate for wasm-bindgen-downcast")
    (description
     "This package provides The proc-macro crate for wasm-bindgen-downcast.")
    (license (list license:expat license:asl2.0))))

(define-public rust-wasm-bindgen-downcast-0.1
  (package
    (name "rust-wasm-bindgen-downcast")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasm-bindgen-downcast" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0h23lhqjrrqvi5fhm4wf7r0gdvariyk6p5f0w5y6xjmw8dnh5b2x"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-js-sys" ,rust-js-sys-0.3)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-wasm-bindgen" ,rust-wasm-bindgen-0.2)
                       ("rust-wasm-bindgen-downcast-macros" ,rust-wasm-bindgen-downcast-macros-0.1))))
    (home-page "https://wasmer.io/")
    (synopsis
     "Downcast a JavaScript wrapper generated by `wasm-bindgen` back to its original struct")
    (description
     "This package provides Downcast a @code{JavaScript} wrapper generated by `wasm-bindgen` back to its
original struct.")
    (license (list license:expat license:asl2.0))))

(define-public rust-rusty-jsc-sys-0.1
  (package
    (name "rust-rusty-jsc-sys")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rusty_jsc_sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1pyqm75fdykljz01afwfvfxcn5h2nzzy1gribs332wr6gaml6x4j"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-pkg-config" ,rust-pkg-config-0.3))))
    (home-page "https://github.com/wasmerio/rusty_jsc")
    (synopsis "Raw bindings for the JavaScriptCore engine")
    (description
     "This package provides Raw bindings for the @code{JavaScriptCore} engine.")
    (license license:expat)))

(define-public rust-rusty-jsc-macros-0.1
  (package
    (name "rust-rusty-jsc-macros")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rusty_jsc_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0wa0vh8vj2vd14rx1fi8g1pwj90pikk8ga1dvnp75lf2gh4hf93v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-pkg-config" ,rust-pkg-config-0.3)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/wasmerio/rusty_jsc")
    (synopsis "Macros for rusty_jsc")
    (description "This package provides Macros for rusty_jsc.")
    (license license:expat)))

(define-public rust-rusty-jsc-0.1
  (package
    (name "rust-rusty-jsc")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rusty_jsc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1l7xvag2mz884xpm3m43byvjka0cwwhddg30mv1gxi49qj5873sg"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-rusty-jsc-macros" ,rust-rusty-jsc-macros-0.1)
                       ("rust-rusty-jsc-sys" ,rust-rusty-jsc-sys-0.1))))
    (home-page "https://github.com/wasmerio/rusty_jsc")
    (synopsis "Rust bindings for the JavaScriptCore engine")
    (description
     "This package provides Rust bindings for the @code{JavaScriptCore} engine.")
    (license license:expat)))

(define-public rust-more-asserts-0.2
  (package
    (name "rust-more-asserts")
    (version "0.2.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "more-asserts" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "12b3fypg2sr4qmxy6wiyx6k9sdg573f5ij98cdmbrg00whnyqhvq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/thomcc/rust-more-asserts")
    (synopsis
     "Small library providing additional assert_* and debug_assert_* macros")
    (description
     "This package provides Small library providing additional assert_* and debug_assert_* macros.")
    (license license:cc0)))

(define-public rust-wasmer-3
  (package
    (name "rust-wasmer")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "wasmer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lnxxx296x8m7zwyccdhickksjj1d66vc0yari3yswfbv3nfvjkq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytes" ,rust-bytes-1)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-derivative" ,rust-derivative-2)
                       ("rust-hashbrown" ,rust-hashbrown-0.11)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-js-sys" ,rust-js-sys-0.3)
                       ("rust-more-asserts" ,rust-more-asserts-0.2)
                       ("rust-rustc-demangle" ,rust-rustc-demangle-0.1)
                       ("rust-rusty-jsc" ,rust-rusty-jsc-0.1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-wasm-bindgen" ,rust-serde-wasm-bindgen-0.4)
                       ("rust-target-lexicon" ,rust-target-lexicon-0.12)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-wasm-bindgen" ,rust-wasm-bindgen-0.2)
                       ("rust-wasm-bindgen-downcast" ,rust-wasm-bindgen-downcast-0.1)
                       ("rust-wasmer-compiler" ,rust-wasmer-compiler-3)
                       ("rust-wasmer-compiler-cranelift" ,rust-wasmer-compiler-cranelift-3)
                       ("rust-wasmer-compiler-llvm" ,rust-wasmer-compiler-llvm-3)
                       ("rust-wasmer-compiler-singlepass" ,rust-wasmer-compiler-singlepass-3)
                       ("rust-wasmer-derive" ,rust-wasmer-derive-3)
                       ("rust-wasmer-types" ,rust-wasmer-types-3)
                       ("rust-wasmer-vm" ,rust-wasmer-vm-3)
                       ("rust-wasmparser" ,rust-wasmparser-0.95)
                       ("rust-wasmparser" ,rust-wasmparser-0.83)
                       ("rust-wat" ,rust-wat-1)
                       ("rust-winapi" ,rust-winapi-0.3))))
    (home-page "https://wasmer.io/")
    (synopsis "High-performance WebAssembly runtime")
    (description
     "This package provides High-performance @code{WebAssembly} runtime.")
    (license license:expat)))

(define-public rust-is-macro-0.3
  (package
    (name "rust-is-macro")
    (version "0.3.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "is-macro" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "13r84nzn5zqxf8pqjhpj2pr0vkjnam1iwnnbmimr05rpq6ymma2r"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-inflector" ,rust-inflector-0.11)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/dudykr/ddbase")
    (synopsis "Derive methods for using custom enums like Option / Result")
    (description
     "This package provides Derive methods for using custom enums like Option / Result.")
    (license license:asl2.0)))

(define-public rust-swc-css-ast-0.137
  (package
    (name "rust-swc-css-ast")
    (version "0.137.21")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_css_ast" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0dq008cf15cx5afj7msvy7amhqccds7xayp76ygrjqw34lxlaaiw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-is-macro" ,rust-is-macro-0.3)
                       ("rust-rkyv" ,rust-rkyv-0.7)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-string-enum" ,rust-string-enum-0.4)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "AST definitions of css")
    (description "This package provides AST definitions of css.")
    (license license:asl2.0)))

(define-public rust-enumset-derive-0.10
  (package
    (name "rust-enumset-derive")
    (version "0.10.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "enumset_derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0hp25sx7p60ms4xkdkz7x36byd96dhpq7hdxcr4k332x6i6b5hsr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-darling" ,rust-darling-0.20)
                       ("rust-proc-macro-crate" ,rust-proc-macro-crate-3)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/Lymia/enumset")
    (synopsis "An internal helper crate for enumset. Not public API")
    (description
     "This package provides An internal helper crate for enumset.  Not public API.")
    (license (list license:expat license:asl2.0))))

(define-public rust-enumset-1
  (package
    (name "rust-enumset")
    (version "1.1.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "enumset" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "14vjn774rvvs2ac2ykdld3bq8w657wy2f6hcbzpmqxjqjl24nynh"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-enumset-derive" ,rust-enumset-derive-0.10)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/Lymia/enumset")
    (synopsis "library for creating compact sets of enums.")
    (description
     "This package provides a library for creating compact sets of enums.")
    (license (list license:expat license:asl2.0))))

(define-public rust-swc-plugin-runner-0.94
  (package
    (name "rust-swc-plugin-runner")
    (version "0.94.23")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_plugin_runner" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1cw9ri1wy5zbnp284ljk4jvhsj9ings3xgssfsi0i9xmkd59chs6"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-enumset" ,rust-enumset-1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-plugin-proxy" ,rust-swc-plugin-proxy-0.32)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-wasmer" ,rust-wasmer-3)
                       ("rust-wasmer-cache" ,rust-wasmer-cache-3)
                       ("rust-wasmer-compiler-cranelift" ,rust-wasmer-compiler-cranelift-3)
                       ("rust-wasmer-wasix" ,rust-wasmer-wasix-0.4))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis
     "Runner for swc plugins. This crate is INTERNAL crate and used by swc itself")
    (description
     "This package provides Runner for swc plugins.  This crate is INTERNAL crate and used by swc itself.")
    (license license:asl2.0)))

(define-public rust-swc-plugin-proxy-0.32
  (package
    (name "rust-swc-plugin-proxy")
    (version "0.32.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_plugin_proxy" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ab62mq6nibai9zc78yqmy6lr90vi6b8b34l99az33kfcadyiyqc"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-better-scoped-tls" ,rust-better-scoped-tls-0.1)
                       ("rust-rkyv" ,rust-rkyv-0.7)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-trace-macro" ,rust-swc-trace-macro-0.1)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Proxy structs to the hosts original structs for the plugin")
    (description
     "This package provides Proxy structs to the hosts original structs for the plugin.")
    (license license:asl2.0)))

(define-public rust-swc-node-comments-0.18
  (package
    (name "rust-swc-node-comments")
    (version "0.18.22")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_node_comments" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jkgfqpi2ypk0q2m8qxk5vhwsxk42yv24ypvkzb19zjlwpqy71ns"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-dashmap" ,rust-dashmap-5)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Implementation of Comments of swc_common")
    (description
     "This package provides Implementation of Comments of swc_common.")
    (license license:asl2.0)))

(define-public rust-supports-unicode-1
  (package
    (name "rust-supports-unicode")
    (version "1.0.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "supports-unicode" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1cpq6mbixlpdibwx203p6qh7kpzqy9yin7y5ird14ys1bgj4bfd8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-atty" ,rust-atty-0.2))))
    (home-page "https://github.com/zkat/supports-unicode")
    (synopsis "Detects whether a terminal supports unicode")
    (description
     "This package provides Detects whether a terminal supports unicode.")
    (license license:asl2.0)))

(define-public rust-supports-hyperlinks-1
  (package
    (name "rust-supports-hyperlinks")
    (version "1.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "supports-hyperlinks" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "01k4rr0p8fvgc8qwy8bnlr8gf7gk8psb796vg2fwq7phqpvk82sr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-atty" ,rust-atty-0.2))))
    (home-page "https://github.com/zkat/supports-hyperlinks")
    (synopsis "Detects whether a terminal supports rendering hyperlinks")
    (description
     "This package provides Detects whether a terminal supports rendering hyperlinks.")
    (license license:asl2.0)))

(define-public rust-supports-color-1
  (package
    (name "rust-supports-color")
    (version "1.3.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "supports-color" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0vqdhwc3yf1bv1xbaz5d8p2brmlv1ap4fhwg8pfjzr3yrbrgm9lb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-atty" ,rust-atty-0.2)
                       ("rust-is-ci" ,rust-is-ci-1))))
    (home-page "https://github.com/zkat/supports-color")
    (synopsis
     "Detects whether a terminal supports color, and gives details about that support")
    (description
     "This package provides Detects whether a terminal supports color, and gives details about that support.")
    (license license:asl2.0)))

(define-public rust-miette-derive-4
  (package
    (name "rust-miette-derive")
    (version "4.7.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "miette-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0k0ph38lxzqamaabind8463j2k5qjg6jhhbcdrg1pkqvfrdw8nvb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/zkat/miette")
    (synopsis "Derive macros for miette. Like `thiserror` for Diagnostics")
    (description
     "This package provides Derive macros for miette.  Like `thiserror` for Diagnostics.")
    (license license:asl2.0)))

(define-public rust-miette-4
  (package
    (name "rust-miette")
    (version "4.7.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "miette" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1365xpl8l66lsvn6bk4mhbpxf5gciiazj4apyiaqn87r8jg3540w"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-atty" ,rust-atty-0.2)
                       ("rust-backtrace" ,rust-backtrace-0.3)
                       ("rust-miette-derive" ,rust-miette-derive-4)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-owo-colors" ,rust-owo-colors-3)
                       ("rust-supports-color" ,rust-supports-color-1)
                       ("rust-supports-hyperlinks" ,rust-supports-hyperlinks-1)
                       ("rust-supports-unicode" ,rust-supports-unicode-1)
                       ("rust-terminal-size" ,rust-terminal-size-0.1)
                       ("rust-textwrap" ,rust-textwrap-0.15)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-unicode-width" ,rust-unicode-width-0.1))))
    (home-page "https://github.com/zkat/miette")
    (synopsis
     "Fancy diagnostic reporting library and protocol for us mere mortals who aren't compiler hackers")
    (description
     "This package provides Fancy diagnostic reporting library and protocol for us mere mortals who aren't
compiler hackers.")
    (license license:asl2.0)))

(define-public rust-swc-error-reporters-0.15
  (package
    (name "rust-swc-error-reporters")
    (version "0.15.22")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_error_reporters" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "00izdpjnzd2kn59jlq2b6i2hm0xhls63j156ym83wpswx2zx7i8r"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-miette" ,rust-miette-4)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-swc-common" ,rust-swc-common-0.31))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Error reporter implementations for the swc project")
    (description
     "This package provides Error reporter implementations for the swc project.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-typescript-0.176
  (package
    (name "rust-swc-ecma-transforms-typescript")
    (version "0.176.30")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_typescript" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0hv5w9qxlqawixbx23ra7b0aqfp9zlfp8g2ppv581jvzkbzy6m74"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-react" ,rust-swc-ecma-transforms-react-0.172)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "rust port of babel and closure compiler")
    (description
     "This package provides rust port of babel and closure compiler.")
    (license license:asl2.0)))

(define-public rust-sha-1-0.10
  (package
    (name "rust-sha-1")
    (version "0.10.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sha-1" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03zag8zk4qlv40n2yryddapv5yxkam3hdr7n53d8qrzr2gali3q2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-cpufeatures" ,rust-cpufeatures-0.2)
                       ("rust-digest" ,rust-digest-0.10)
                       ("rust-sha1-asm" ,rust-sha1-asm-0.5))))
    (home-page "https://github.com/RustCrypto/hashes")
    (synopsis
     "SHA-1 hash function. This crate is deprecated! Use the sha1 crate instead")
    (description
     "This package provides SHA-1 hash function.  This crate is deprecated! Use the sha1 crate instead.")
    (license (list license:expat license:asl2.0))))

(define-public rust-swc-ecma-transforms-react-0.172
  (package
    (name "rust-swc-ecma-transforms-react")
    (version "0.172.29")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_react" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0i1wfwiaqx22jp111lxabf2qk03zkl8haarcsv7l5l22nc46fnqj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-base64" ,rust-base64-0.13)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-sha-1" ,rust-sha-1-0.10)
                       ("rust-string-enum" ,rust-string-enum-0.4)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-config" ,rust-swc-config-0.1)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-macros" ,rust-swc-ecma-transforms-macros-0.5)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "rust port of babel and closure compiler")
    (description
     "This package provides rust port of babel and closure compiler.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-proposal-0.160
  (package
    (name "rust-swc-ecma-transforms-proposal")
    (version "0.160.25")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_proposal" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ylvdkm05mj1519744h1wwky5hxmyf7p3y66ym9srdcns11lyi8g"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-either" ,rust-either-1)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-loader" ,rust-swc-ecma-loader-0.43)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-classes" ,rust-swc-ecma-transforms-classes-0.115)
                       ("rust-swc-ecma-transforms-macros" ,rust-swc-ecma-transforms-macros-0.5)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "rust port of babel and closure compiler")
    (description
     "This package provides rust port of babel and closure compiler.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-module-0.169
  (package
    (name "rust-swc-ecma-transforms-module")
    (version "0.169.27")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_module" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "11qcn7s1qr2gj5l5f8qfmbyxmf8a27yif3v89jh2parcnc8mxa1d"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-anyhow" ,rust-anyhow-1)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-inflector" ,rust-inflector-0.11)
                       ("rust-is-macro" ,rust-is-macro-0.2)
                       ("rust-path-clean" ,rust-path-clean-0.1)
                       ("rust-pathdiff" ,rust-pathdiff-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-cached" ,rust-swc-cached-0.3)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-loader" ,rust-swc-ecma-loader-0.43)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "rust port of babel and closure compiler")
    (description
     "This package provides rust port of babel and closure compiler.")
    (license license:asl2.0)))

(define-public rust-swc-trace-macro-0.1
  (package
    (name "rust-swc-trace-macro")
    (version "0.1.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_trace_macro" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1fjjw1jrynl29k32fbayz444pqbvjf0shf0rsr7q5ljx12v1k5zz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Proc macro for performance trace of swc")
    (description
     "This package provides Proc macro for performance trace of swc.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-classes-0.115
  (package
    (name "rust-swc-ecma-transforms-classes")
    (version "0.115.20")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_classes" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "123w7mzwqq6x6im7v9syhjvx8y2fm781y9rx1agjb89l9dcry2d3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Helper for transforms for the swc project")
    (description
     "This package provides Helper for transforms for the swc project.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-compat-0.152
  (package
    (name "rust-swc-ecma-transforms-compat")
    (version "0.152.23")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_compat" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0h29mzf6zp3gcj92awgqs7c444vwkh96ll3x7f6c935xh4hbq6y6"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-is-macro" ,rust-is-macro-0.2)
                       ("rust-num-bigint" ,rust-num-bigint-0.4)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-config" ,rust-swc-config-0.1)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-classes" ,rust-swc-ecma-transforms-classes-0.115)
                       ("rust-swc-ecma-transforms-macros" ,rust-swc-ecma-transforms-macros-0.5)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-swc-trace-macro" ,rust-swc-trace-macro-0.1)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "rust port of babel and closure compiler")
    (description
     "This package provides rust port of babel and closure compiler.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-0.217
  (package
    (name "rust-swc-ecma-transforms")
    (version "0.217.31")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0hd0zplv2bra0rqzzisa02090kdzakddkm2zhiqfnk6jckfiy7yv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-compat" ,rust-swc-ecma-transforms-compat-0.152)
                       ("rust-swc-ecma-transforms-module" ,rust-swc-ecma-transforms-module-0.169)
                       ("rust-swc-ecma-transforms-optimization" ,rust-swc-ecma-transforms-optimization-0.186)
                       ("rust-swc-ecma-transforms-proposal" ,rust-swc-ecma-transforms-proposal-0.160)
                       ("rust-swc-ecma-transforms-react" ,rust-swc-ecma-transforms-react-0.172)
                       ("rust-swc-ecma-transforms-typescript" ,rust-swc-ecma-transforms-typescript-0.176)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "rust port of babel and closure compiler")
    (description
     "This package provides rust port of babel and closure compiler.")
    (license license:asl2.0)))

(define-public rust-static-map-macro-0.2
  (package
    (name "rust-static-map-macro")
    (version "0.2.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "static-map-macro" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16c5xw3krap14d9mhb3z8vkh3x9k2kkf543q05dhixy9zfcdaqmq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-pmutil" ,rust-pmutil-0.5)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/dudykr/ddbase")
    (synopsis "Macro to create a stack-alocated map")
    (description "This package provides Macro to create a stack-alocated map.")
    (license license:expat)))

(define-public rust-st-map-0.1
  (package
    (name "rust-st-map")
    (version "0.1.8")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "st-map" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1vdmg8sr3iynkblcd97pl4yslisdnn7lgm4dlpab0xph6lc8k7gh"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-static-map-macro" ,rust-static-map-macro-0.2))))
    (home-page "https://github.com/dudykr/ddbase")
    (synopsis "Runtime for a stack-alocated map")
    (description "This package provides Runtime for a stack-alocated map.")
    (license license:expat)))

(define-public rust-static-map-macro-0.3
  (package
    (name "rust-static-map-macro")
    (version "0.3.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "static-map-macro" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0m44m5ny5p9rd0mkxkgsfynljla22ic8k1qnp78h6fhfgvm87x4w"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/dudykr/ddbase")
    (synopsis "Macro to create a stack-alocated map")
    (description "This package provides Macro to create a stack-alocated map.")
    (license license:asl2.0)))

(define-public rust-st-map-0.2
  (package
    (name "rust-st-map")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "st-map" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "18c9d9x6kaa70mnn30zssyprqj37vyr70ls98s9l0ff8bi75r2la"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-static-map-macro" ,rust-static-map-macro-0.3))))
    (home-page "https://github.com/dudykr/ddbase")
    (synopsis "Runtime for a stack-alocated map")
    (description "This package provides Runtime for a stack-alocated map.")
    (license license:asl2.0)))

(define-public rust-browserslist-rs-0.15
  (package
    (name "rust-browserslist-rs")
    (version "0.15.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "browserslist-rs" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1lz89hzfj0r7j58wpzh311j2x92s02pwirdkbnz1li0ab53bsns0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-anyhow" ,rust-anyhow-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-either" ,rust-either-1)
                       ("rust-getrandom" ,rust-getrandom-0.2)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-itertools" ,rust-itertools-0.12)
                       ("rust-js-sys" ,rust-js-sys-0.3)
                       ("rust-nom" ,rust-nom-7)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-wasm-bindgen" ,rust-serde-wasm-bindgen-0.4)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-string-cache" ,rust-string-cache-0.8)
                       ("rust-string-cache-codegen" ,rust-string-cache-codegen-0.5)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-wasm-bindgen" ,rust-wasm-bindgen-0.2))))
    (home-page "https://github.com/browserslist/browserslist-rs")
    (synopsis "Rust-ported Browserslist")
    (description "This package provides Rust-ported Browserslist.")
    (license license:expat)))

(define-public rust-preset-env-base-0.4
  (package
    (name "rust-preset-env-base")
    (version "0.4.13")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "preset_env_base" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ysjmzmd3ar3ljggwjskl1lxrgy0b25zfskizbay0flmg5bd3k08"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-anyhow" ,rust-anyhow-1)
                       ("rust-browserslist-rs" ,rust-browserslist-rs-0.15)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-from-variant" ,rust-from-variant-0.1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-semver" ,rust-semver-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-st-map" ,rust-st-map-0.2)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Common logic for targetting vairous browsers")
    (description
     "This package provides Common logic for targetting vairous browsers.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-preset-env-0.194
  (package
    (name "rust-swc-ecma-preset-env")
    (version "0.194.32")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_preset_env" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1dlhl456ifjpwcsfw8bd17nwwpr6wr6ycd2rywn897kd3zpa3x04"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-anyhow" ,rust-anyhow-1)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-preset-env-base" ,rust-preset-env-base-0.4)
                       ("rust-semver" ,rust-semver-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-st-map" ,rust-st-map-0.1)
                       ("rust-string-enum" ,rust-string-enum-0.4)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-transforms" ,rust-swc-ecma-transforms-0.217)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "preset-env for the swc")
    (description "This package provides preset-env for the swc.")
    (license license:asl2.0)))

(define-public rust-swc-timer-0.19
  (package
    (name "rust-swc-timer")
    (version "0.19.25")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_timer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1f2p94d3xk638kymn2ji0r75r482xcir34yizmwyiisinlnmij0w"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Timings for swc")
    (description "This package provides Timings for swc.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-usage-analyzer-0.12
  (package
    (name "rust-swc-ecma-usage-analyzer")
    (version "0.12.14")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_usage_analyzer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1xyjkzs1rknfwl9s29l6fkrsh91vmav8c9i4y7303g96h0hiph6i"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-swc-timer" ,rust-swc-timer-0.19)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "EcmaScript variable usage analyzer")
    (description
     "This package provides @code{EcmaScript} variable usage analyzer.")
    (license license:asl2.0)))

(define-public rust-swc-fast-graph-0.19
  (package
    (name "rust-swc-fast-graph")
    (version "0.19.22")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_fast_graph" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1h0790i4nfgnmsvifwjdz7p63mmkxdajbhxvwlrbk8ryxfsxhkpr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-indexmap" ,rust-indexmap-1)
                       ("rust-petgraph" ,rust-petgraph-0.6)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-swc-common" ,rust-swc-common-0.31))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Faster version of petgraph")
    (description "This package provides Faster version of petgraph.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-macros-0.5
  (package
    (name "rust-swc-ecma-transforms-macros")
    (version "0.5.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1assn76kd11lld5vjn9i1ng7qryyspbb6cyn2zj423hymnnis2jh"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-swc-macros-common" ,rust-swc-macros-common-0.3)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Macros for swc_ecma_transforms")
    (description "This package provides Macros for swc_ecma_transforms.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-optimization-0.186
  (package
    (name "rust-swc-ecma-transforms-optimization")
    (version "0.186.31")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_optimization" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0n0s59izjfw07781naxhbn9dwf3b9sjkyml1aldj9z8y9szfkq2i"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-petgraph" ,rust-petgraph-0.6)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-macros" ,rust-swc-ecma-transforms-macros-0.5)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-swc-fast-graph" ,rust-swc-fast-graph-0.19)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "rust port of babel and closure compiler")
    (description
     "This package provides rust port of babel and closure compiler.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-transforms-base-0.126
  (package
    (name "rust-swc-ecma-transforms-base")
    (version "0.126.20")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_transforms_base" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "06c5glr1x5i9vccx4zczfa3bwyjisd62pcmhgpqfv0q5j6cksbf2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-better-scoped-tls" ,rust-better-scoped-tls-0.1)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-phf" ,rust-phf-0.10)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "rust port of babel and closure compiler")
    (description
     "This package provides rust port of babel and closure compiler.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-parser-0.133
  (package
    (name "rust-swc-ecma-parser")
    (version "0.133.14")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_parser" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1rf8acz2s1d1zm5vcan5clqlx1qagqsfzf5va34yr14l08jf0r8j"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-either" ,rust-either-1)
                       ("rust-lexical" ,rust-lexical-6)
                       ("rust-num-bigint" ,rust-num-bigint-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-smartstring" ,rust-smartstring-1)
                       ("rust-stacker" ,rust-stacker-0.1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-typed-arena" ,rust-typed-arena-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Feature-complete es2019 parser")
    (description "This package provides Feature-complete es2019 parser.")
    (license license:asl2.0)))

(define-public rust-ryu-js-0.2
  (package
    (name "rust-ryu-js")
    (version "0.2.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "ryu-js" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0zqf0ypy6dl26bz4aigqvrvn6wfqx4iy9mi2i9km6kgdphkgq635"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-no-panic" ,rust-no-panic-0.1))))
    (home-page "https://github.com/boa-dev/ryu-js")
    (synopsis "Fast floating point to string conversion, ECMAScript compliant")
    (description
     "This package provides Fast floating point to string conversion, ECMAScript compliant.")
    (license (list license:asl2.0 license:boost1.0))))

(define-public rust-swc-ecma-minifier-0.180
  (package
    (name "rust-swc-ecma-minifier")
    (version "0.180.36")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_minifier" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1xrsirvdzr7rwp7krgcka5gj2gvyqgf1cam0i03skfg25zcj66n3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-backtrace" ,rust-backtrace-0.3)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-num-bigint" ,rust-num-bigint-0.4)
                       ("rust-num-cpus" ,rust-num-cpus-1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-pretty-assertions" ,rust-pretty-assertions-1)
                       ("rust-radix-fmt" ,rust-radix-fmt-1)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-ryu-js" ,rust-ryu-js-0.2)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-cached" ,rust-swc-cached-0.3)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-config" ,rust-swc-config-0.1)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-codegen" ,rust-swc-ecma-codegen-0.138)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-optimization" ,rust-swc-ecma-transforms-optimization-0.186)
                       ("rust-swc-ecma-usage-analyzer" ,rust-swc-ecma-usage-analyzer-0.12)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-swc-timer" ,rust-swc-timer-0.19)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "EcmaScript code minifier")
    (description "This package provides @code{EcmaScript} code minifier.")
    (license license:asl2.0)))

(define-public rust-lru-0.10
  (package
    (name "rust-lru")
    (version "0.10.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "lru" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0w5n2sgh66ac8ihqv6688mlm7zb3ks18jlbzpbhwgw3x8jp8z3ki"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-hashbrown" ,rust-hashbrown-0.13))))
    (home-page "https://github.com/jeromefroe/lru-rs")
    (synopsis "LRU cache implementation")
    (description "This package provides a LRU cache implementation.")
    (license license:expat)))

(define-public rust-swc-ecma-loader-0.43
  (package
    (name "rust-swc-ecma-loader")
    (version "0.43.24")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_loader" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1pih0dn8x4xslpcz6zgykw77xl0sf5plsv6ax9ic2z9wkks1arac"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-lru" ,rust-lru-0.10)
                       ("rust-normpath" ,rust-normpath-0.2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-path-clean" ,rust-path-clean-0.1)
                       ("rust-pathdiff" ,rust-pathdiff-0.2)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-swc-cached" ,rust-swc-cached-0.3)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "General ecmascript loader used for transforms")
    (description
     "This package provides General ecmascript loader used for transforms.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-lints-0.81
  (package
    (name "rust-swc-ecma-lints")
    (version "0.81.20")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_lints" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1lf9whcy4w8jgapg6qjrm1623i77wz2k5pdg7lalldglma0mmlin"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-auto-impl" ,rust-auto-impl-0.5)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-config" ,rust-swc-config-0.1)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Linter for the swc project")
    (description "This package provides Linter for the swc project.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-visit-0.89
  (package
    (name "rust-swc-ecma-visit")
    (version "0.89.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_visit" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1pp9bg7qy1m0zyzj6549rc240v2swqnvvhdhh3cfr5mlb1nds4nj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-num-bigint" ,rust-num-bigint-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-visit" ,rust-swc-visit-0.5)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Visitors for swc ecmascript nodes which works on stable rustc")
    (description
     "This package provides Visitors for swc ecmascript nodes which works on stable rustc.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-utils-0.116
  (package
    (name "rust-swc-ecma-utils")
    (version "0.116.14")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_utils" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0mn2d73fcsyhcsxm9jvdhjw0drwhs5ihj6ws5pmr1nsc5in9w3ad"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-indexmap" ,rust-indexmap-1)
                       ("rust-num-cpus" ,rust-num-cpus-1)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-unicode-id" ,rust-unicode-id-0.3))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Utilities for swc ecmascript ast nodes")
    (description
     "This package provides Utilities for swc ecmascript ast nodes.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-ext-transforms-0.102
  (package
    (name "rust-swc-ecma-ext-transforms")
    (version "0.102.14")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_ext_transforms" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0gj21wcgkjhqscl1zpzs2xbd9695hxdpdn3razs9myik01z054jb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-phf" ,rust-phf-0.10)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Extensions for @swc/core (nodejs)")
    (description "This package provides Extensions for @@swc/core (nodejs).")
    (license license:asl2.0)))

(define-public rust-swc-ecma-codegen-macros-0.7
  (package
    (name "rust-swc-ecma-codegen-macros")
    (version "0.7.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_codegen_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jhbgajqy0b77v3a6rqvpbmil321wgaqsm5d7x7n6f6v6vgap7w5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-swc-macros-common" ,rust-swc-macros-common-0.3)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Macros for swc_ecma_codegen")
    (description "This package provides Macros for swc_ecma_codegen.")
    (license license:asl2.0)))

(define-public rust-swc-ecma-codegen-0.138
  (package
    (name "rust-swc-ecma-codegen")
    (version "0.138.17")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_codegen" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "10ls4icqybd487pgzyzb2y4ic5n98n427dw2gh6z0z5yhpm2rgl0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-memchr" ,rust-memchr-2)
                       ("rust-num-bigint" ,rust-num-bigint-0.4)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-sourcemap" ,rust-sourcemap-6)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-codegen-macros" ,rust-swc-ecma-codegen-macros-0.7)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Ecmascript code generator for the swc project")
    (description
     "This package provides Ecmascript code generator for the swc project.")
    (license license:asl2.0)))

(define-public rust-string-enum-0.4
  (package
    (name "rust-string-enum")
    (version "0.4.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "string_enum" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "142vajhqzglrgykyqi17v5w28qsw0px280ljszkmghpbi8q87qq5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-swc-macros-common" ,rust-swc-macros-common-0.3)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "String based enum")
    (description "This package provides String based enum.")
    (license license:asl2.0)))

(define-public rust-bytecheck-0.6
  (package
    (name "rust-bytecheck")
    (version "0.6.10")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "bytecheck" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "07vxs7y441f7d6mjzmli80ykmfajwk9jqci549b29sr319j13zhk"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytecheck-derive" ,rust-bytecheck-derive-0.6)
                       ("rust-ptr-meta" ,rust-ptr-meta-0.1)
                       ("rust-simdutf8" ,rust-simdutf8-0.1)
                       ("rust-uuid" ,rust-uuid-1))))
    (home-page "https://github.com/rkyv/bytecheck")
    (synopsis "Derive macro for bytecheck")
    (description "This package provides Derive macro for bytecheck.")
    (license license:expat)))

(define-public rust-rkyv-0.7
  (package
    (name "rust-rkyv")
    (version "0.7.41")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rkyv" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0gps7i69lpjjhkcnf601i2fmw8y0rjran2l8h7h7mw0733crwj91"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-bitvec" ,rust-bitvec-1)
                       ("rust-bytecheck" ,rust-bytecheck-0.6)
                       ("rust-hashbrown" ,rust-hashbrown-0.12)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-ptr-meta" ,rust-ptr-meta-0.1)
                       ("rust-rend" ,rust-rend-0.4)
                       ("rust-rkyv-derive" ,rust-rkyv-derive-0.7)
                       ("rust-seahash" ,rust-seahash-4)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-tinyvec" ,rust-tinyvec-1)
                       ("rust-uuid" ,rust-uuid-1))))
    (home-page "https://github.com/rkyv/rkyv")
    (synopsis "Zero-copy deserialization framework for Rust")
    (description
     "This package provides Zero-copy deserialization framework for Rust.")
    (license license:expat)))

(define-public rust-is-macro-0.2
  (package
    (name "rust-is-macro")
    (version "0.2.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "is-macro" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "083v9iqim0cf7rlj09gmks82qv6fx77z3i6595x4fxwv2ag0fzca"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-inflector" ,rust-inflector-0.11)
                       ("rust-pmutil" ,rust-pmutil-0.5)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/dudykr/ddbase")
    (synopsis "Derive methods for using custom enums like Option / Result")
    (description
     "This package provides Derive methods for using custom enums like Option / Result.")
    (license license:expat)))

(define-public rust-swc-ecma-ast-0.103
  (package
    (name "rust-swc-ecma-ast")
    (version "0.103.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_ecma_ast" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0i81brdjyabianxf7dnxb2gq56w5i81xjjmnh53mc6fhljza5y9c"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arbitrary" ,rust-arbitrary-1)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-bytecheck" ,rust-bytecheck-0.6)
                       ("rust-is-macro" ,rust-is-macro-0.2)
                       ("rust-num-bigint" ,rust-num-bigint-0.4)
                       ("rust-rkyv" ,rust-rkyv-0.7)
                       ("rust-scoped-tls" ,rust-scoped-tls-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-string-enum" ,rust-string-enum-0.4)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-unicode-id" ,rust-unicode-id-0.3))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Ecmascript ast")
    (description "This package provides Ecmascript ast.")
    (license license:asl2.0)))

(define-public rust-swc-config-macro-0.1
  (package
    (name "rust-swc-config-macro")
    (version "0.1.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_config_macro" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16c3mrmy67dd7m3kv7fckcv2s5z0l2x4ijpmaidskha2j09mcpvw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-swc-macros-common" ,rust-swc-macros-common-0.3)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Macros to prevent mistakes")
    (description "This package provides Macros to prevent mistakes.")
    (license license:asl2.0)))

(define-public rust-unicode-id-start-1
  (package
    (name "rust-unicode-id-start")
    (version "1.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "unicode-id-start" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1spdy83vvcv5a0cp2hlpy83bns4g7ncyfgnyqj6ar8h7jvv84f5w"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/Boshen/unicode-id-start")
    (synopsis
     "Determine whether characters have the ID_Start or ID_Continue properties according to Unicode Standard Annex #31")
    (description
     "This package provides Determine whether characters have the ID_Start or ID_Continue properties
according to Unicode Standard Annex #31.")
    (license (list))))

(define-public rust-outref-0.1
  (package
    (name "rust-outref")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "outref" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1x61h7dl1cc6cj2f3zsalr8d98v0cw6497sykwxf74wjmqljh8kz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/Nugine/outref")
    (synopsis "Out reference")
    (description "This package provides Out reference.")
    (license license:expat)))

(define-public rust-simd-abstraction-0.7
  (package
    (name "rust-simd-abstraction")
    (version "0.7.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "simd-abstraction" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "11v9hy8qg0b4qypz2p75ijv41ln1rssk6qilz0gwbbfaayfb5bcw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-outref" ,rust-outref-0.1))))
    (home-page "https://github.com/Nugine/simd")
    (synopsis "Abstract SIMD instruction sets")
    (description "This package provides Abstract SIMD instruction sets.")
    (license license:expat)))

(define-public rust-base64-simd-0.7
  (package
    (name "rust-base64-simd")
    (version "0.7.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "base64-simd" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1mg5ayj5z7imfyv06fhzi5rw289gv5yrfakxzsad22zz786d47bq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-simd-abstraction" ,rust-simd-abstraction-0.7))))
    (home-page "https://github.com/Nugine/simd")
    (synopsis "SIMD-accelerated base64 encoding and decoding")
    (description
     "This package provides SIMD-accelerated base64 encoding and decoding.")
    (license license:expat)))

(define-public rust-sourcemap-8
  (package
    (name "rust-sourcemap")
    (version "8.0.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sourcemap" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1m4nbgsgdlqrfby6qz114hc2ng4gxnas53kp2cvgknfax2wl1390"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-base64-simd" ,rust-base64-simd-0.7)
                       ("rust-bitvec" ,rust-bitvec-1)
                       ("rust-data-encoding" ,rust-data-encoding-2)
                       ("rust-debugid" ,rust-debugid-0.8)
                       ("rust-if-chain" ,rust-if-chain-1)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-rustc-version" ,rust-rustc-version-0.2)
                       ("rust-scroll" ,rust-scroll-0.10)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-unicode-id-start" ,rust-unicode-id-start-1)
                       ("rust-url" ,rust-url-2))))
    (home-page "https://github.com/getsentry/rust-sourcemap")
    (synopsis "Basic sourcemap handling for Rust")
    (description "This package provides Basic sourcemap handling for Rust.")
    (license license:bsd-3)))

(define-public rust-indexmap-2
  (package
    (name "rust-indexmap")
    (version "2.2.6")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "indexmap" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "09hgwi2ig0wyj5rjziia76zmhgfj95k0jb4ic3iiawm4vlavg3qn"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arbitrary" ,rust-arbitrary-1)
                       ("rust-borsh" ,rust-borsh-1)
                       ("rust-equivalent" ,rust-equivalent-1)
                       ("rust-hashbrown" ,rust-hashbrown-0.14)
                       ("rust-quickcheck" ,rust-quickcheck-1)
                       ("rust-rayon" ,rust-rayon-1)
                       ("rust-rustc-rayon" ,rust-rustc-rayon-0.5)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/indexmap-rs/indexmap")
    (synopsis "hash table with consistent order and fast iteration.")
    (description
     "This package provides a hash table with consistent order and fast iteration.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-swc-config-0.1
  (package
    (name "rust-swc-config")
    (version "0.1.14")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_config" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "157xwcbqdi08alb6vaa6nkp84xya102vnn1mn07gwdmib88pxdl4"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sourcemap" ,rust-sourcemap-8)
                       ("rust-swc-cached" ,rust-swc-cached-0.3)
                       ("rust-swc-config-macro" ,rust-swc-config-macro-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Types for configuring swc")
    (description "This package provides Types for configuring swc.")
    (license license:asl2.0)))

(define-public rust-swc-visit-macros-0.5
  (package
    (name "rust-swc-visit-macros")
    (version "0.5.13")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_visit_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0h8bx505x870vdqd5rk8h1jkq6g8hczllxwarrh9rwsr1627v04j"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-inflector" ,rust-inflector-0.11)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-swc-macros-common" ,rust-swc-macros-common-0.3)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Visitor generator for stable rustc")
    (description "This package provides Visitor generator for stable rustc.")
    (license license:asl2.0)))

(define-public rust-swc-visit-0.5
  (package
    (name "rust-swc-visit")
    (version "0.5.14")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_visit" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1fbig3pp2sf1h932wlnfzbsmlsl9q02r9m7ahd2r7jrxd3z12g84"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-either" ,rust-either-1)
                       ("rust-swc-visit-macros" ,rust-swc-visit-macros-0.5))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Visitor generator for stable rustc")
    (description "This package provides Visitor generator for stable rustc.")
    (license license:asl2.0)))

(define-public rust-swc-eq-ignore-macros-0.1
  (package
    (name "rust-swc-eq-ignore-macros")
    (version "0.1.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_eq_ignore_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "15q42gir44xarn22n89nv4ny7rxb1rfb5iais71j1li9zzf0mnv3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Macros for EqIgnoreSpan and TypeEq")
    (description
     "This package provides Macros for @code{EqIgnoreSpan} and @code{TypeEq}.")
    (license license:asl2.0)))

(define-public rust-from-variant-0.1
  (package
    (name "rust-from-variant")
    (version "0.1.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "from_variant" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1m4fs1r3k0yyj4f2nilvsr5h1ndwivyh0bbmfiaaz0pb8896y09j"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-swc-macros-common" ,rust-swc-macros-common-0.3)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Automatically derive From impls for enums")
    (description
     "This package provides Automatically derive From impls for enums.")
    (license license:asl2.0)))

(define-public rust-scoped-tls-1
  (package
    (name "rust-scoped-tls")
    (version "1.0.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "scoped-tls" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "15524h04mafihcvfpgxd8f4bgc3k95aclz8grjkg9a0rxcvn9kz1"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/alexcrichton/scoped-tls")
    (synopsis
     "Library implementation of the standard library's old `scoped_thread_local!`
macro for providing scoped access to thread local storage (TLS) so any type can
be stored into TLS.")
    (description
     "This package provides Library implementation of the standard library's old `scoped_thread_local!`
macro for providing scoped access to thread local storage (TLS) so any type can
be stored into TLS.")
    (license (list license:expat license:asl2.0))))

(define-public rust-better-scoped-tls-0.1
  (package
    (name "rust-better-scoped-tls")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "better_scoped_tls" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ph14yy6h8is61l2sy6vx97knrj5zn9z04daxi5bn1zvng4xqkkr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-scoped-tls" ,rust-scoped-tls-1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "scoped-tls, but with good error message")
    (description
     "This package provides scoped-tls, but with good error message.")
    (license license:asl2.0)))

(define-public rust-swc-macros-common-0.3
  (package
    (name "rust-swc-macros-common")
    (version "0.3.13")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_macros_common" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0izqbdfbacczwikwpsxbkfycx05qb3ad57pnd045cp3vzdxni1pl"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Common utilities for swc macros")
    (description "This package provides Common utilities for swc macros.")
    (license license:asl2.0)))

(define-public rust-ast-node-0.9
  (package
    (name "rust-ast-node")
    (version "0.9.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "ast_node" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ck8i3vwx0qqm20f9kf6xmzg50aqg2dwi4r4f4jqcglv6qmly67r"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-swc-macros-common" ,rust-swc-macros-common-0.3)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Macros for ast nodes")
    (description "This package provides Macros for ast nodes.")
    (license license:asl2.0)))

(define-public rust-swc-common-0.31
  (package
    (name "rust-swc-common")
    (version "0.31.22")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_common" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0a6m73cd7rki71nhd2lqrwpj89kzs3s9417k6g0mjz361jb0zl48"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-anyhow" ,rust-anyhow-1)
                       ("rust-arbitrary" ,rust-arbitrary-1)
                       ("rust-ast-node" ,rust-ast-node-0.9)
                       ("rust-atty" ,rust-atty-0.2)
                       ("rust-better-scoped-tls" ,rust-better-scoped-tls-0.1)
                       ("rust-bytecheck" ,rust-bytecheck-0.6)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-either" ,rust-either-1)
                       ("rust-from-variant" ,rust-from-variant-0.1)
                       ("rust-new-debug-unreachable" ,rust-new-debug-unreachable-1)
                       ("rust-num-bigint" ,rust-num-bigint-0.4)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-rkyv" ,rust-rkyv-0.7)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-siphasher" ,rust-siphasher-0.3)
                       ("rust-sourcemap" ,rust-sourcemap-6)
                       ("rust-string-cache" ,rust-string-cache-0.8)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-eq-ignore-macros" ,rust-swc-eq-ignore-macros-0.1)
                       ("rust-swc-visit" ,rust-swc-visit-0.5)
                       ("rust-termcolor" ,rust-termcolor-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-unicode-width" ,rust-unicode-width-0.1)
                       ("rust-url" ,rust-url-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Common utilities for the swc project")
    (description "This package provides Common utilities for the swc project.")
    (license license:asl2.0)))

(define-public rust-serde-derive-1
  (package
    (name "rust-serde-derive")
    (version "1.0.204")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serde_derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "08p25262mbmhsr2cg0508d5b1wvljj956rvpg0v3qgg6gc8pxkg0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://serde.rs")
    (synopsis "Macros 1.1 implementation of #[derive(Serialize, Deserialize)]")
    (description
     "This package provides Macros 1.1 implementation of #[derive(Serialize, Deserialize)].")
    (license (list license:expat license:asl2.0))))

(define-public rust-anyhow-1
  (package
    (name "rust-anyhow")
    (version "1.0.86")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "anyhow" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1nk301x8qhpdaks6a9zvcp7yakjqnczjmqndbg7vk4494d3d1ldk"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-backtrace" ,rust-backtrace-0.3))))
    (home-page "https://github.com/dtolnay/anyhow")
    (synopsis "Flexible concrete Error type built on std::error::Error")
    (description
     "This package provides Flexible concrete Error type built on std::error::Error.")
    (license (list license:expat license:asl2.0))))

(define-public rust-ahash-0.8
  (package
    (name "rust-ahash")
    (version "0.8.11")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "ahash" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "04chdfkls5xmhp1d48gnjsmglbqibizs3bpbj6rsj604m10si7g8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-atomic-polyfill" ,rust-atomic-polyfill-1)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-const-random" ,rust-const-random-0.1)
                       ("rust-getrandom" ,rust-getrandom-0.2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-version-check" ,rust-version-check-0.9)
                       ("rust-zerocopy" ,rust-zerocopy-0.7))))
    (home-page "https://github.com/tkaitchuck/ahash")
    (synopsis
     "non-cryptographic hash function using AES-NI for high performance")
    (description
     "This package provides a non-cryptographic hash function using AES-NI for high
performance.")
    (license (list license:expat license:asl2.0))))

(define-public rust-swc-cached-0.3
  (package
    (name "rust-swc-cached")
    (version "0.3.20")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_cached" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0p6h9iw3cw1xavkipf5qk3jykbsy2924yi17kk70z1h1qlhn4h43"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-anyhow" ,rust-anyhow-1)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Cached types for swc")
    (description "This package provides Cached types for swc.")
    (license license:asl2.0)))

(define-public rust-rkyv-0.7
  (package
    (name "rust-rkyv")
    (version "0.7.42")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rkyv" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0n2wzwnghkr2ny16c08f5szbkljfqrp3s8fnnb096f011ciwh002"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-bitvec" ,rust-bitvec-1)
                       ("rust-bytecheck" ,rust-bytecheck-0.6)
                       ("rust-hashbrown" ,rust-hashbrown-0.12)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-ptr-meta" ,rust-ptr-meta-0.1)
                       ("rust-rend" ,rust-rend-0.4)
                       ("rust-rkyv-derive" ,rust-rkyv-derive-0.7)
                       ("rust-seahash" ,rust-seahash-4)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-tinyvec" ,rust-tinyvec-1)
                       ("rust-uuid" ,rust-uuid-1))))
    (home-page "https://github.com/rkyv/rkyv")
    (synopsis "Zero-copy deserialization framework for Rust")
    (description
     "This package provides Zero-copy deserialization framework for Rust.")
    (license license:expat)))

(define-public rust-swc-atoms-0.5
  (package
    (name "rust-swc-atoms")
    (version "0.5.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_atoms" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0jg48z60gvn3sbnf7ssd027c5v7pvq9fs57ymka2d9nbglymcm4z"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytecheck" ,rust-bytecheck-0.6)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-rkyv" ,rust-rkyv-0.7)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-string-cache" ,rust-string-cache-0.8)
                       ("rust-string-cache-codegen" ,rust-string-cache-codegen-0.5)
                       ("rust-triomphe" ,rust-triomphe-0.1))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Atoms for the swc project")
    (description "This package provides Atoms for the swc project.")
    (license license:asl2.0)))

(define-public rust-jsonc-parser-0.21
  (package
    (name "rust-jsonc-parser")
    (version "0.21.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "jsonc-parser" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0gblaxf94da7hp9ynmx69r93qpgmbksd3z09b8jq8li3fq7a4mkv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde-json" ,rust-serde-json-1))))
    (home-page "https://github.com/dprint/jsonc-parser")
    (synopsis "JSONC parser")
    (description "This package provides JSONC parser.")
    (license license:expat)))

(define-public rust-swc-0.260
  (package
    (name "rust-swc")
    (version "0.260.48")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0543pr0dmmrn8dc1wnnziygqx39wbh1agzdvjbr8gmr2skzg258c"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.7)
                       ("rust-anyhow" ,rust-anyhow-1)
                       ("rust-base64" ,rust-base64-0.13)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-either" ,rust-either-1)
                       ("rust-indexmap" ,rust-indexmap-1)
                       ("rust-jsonc-parser" ,rust-jsonc-parser-0.21)
                       ("rust-lru" ,rust-lru-0.7)
                       ("rust-napi" ,rust-napi-2)
                       ("rust-napi-derive" ,rust-napi-derive-2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-pathdiff" ,rust-pathdiff-0.2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sourcemap" ,rust-sourcemap-6)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-cached" ,rust-swc-cached-0.3)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-config" ,rust-swc-config-0.1)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-codegen" ,rust-swc-ecma-codegen-0.138)
                       ("rust-swc-ecma-ext-transforms" ,rust-swc-ecma-ext-transforms-0.102)
                       ("rust-swc-ecma-lints" ,rust-swc-ecma-lints-0.81)
                       ("rust-swc-ecma-loader" ,rust-swc-ecma-loader-0.43)
                       ("rust-swc-ecma-minifier" ,rust-swc-ecma-minifier-0.180)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-preset-env" ,rust-swc-ecma-preset-env-0.194)
                       ("rust-swc-ecma-transforms" ,rust-swc-ecma-transforms-0.217)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-compat" ,rust-swc-ecma-transforms-compat-0.152)
                       ("rust-swc-ecma-transforms-optimization" ,rust-swc-ecma-transforms-optimization-0.186)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-swc-error-reporters" ,rust-swc-error-reporters-0.15)
                       ("rust-swc-node-comments" ,rust-swc-node-comments-0.18)
                       ("rust-swc-plugin-proxy" ,rust-swc-plugin-proxy-0.32)
                       ("rust-swc-plugin-runner" ,rust-swc-plugin-runner-0.94)
                       ("rust-swc-timer" ,rust-swc-timer-0.19)
                       ("rust-swc-visit" ,rust-swc-visit-0.5)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-url" ,rust-url-2))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Speedy web compiler")
    (description "This package provides Speedy web compiler.")
    (license license:asl2.0)))

(define-public rust-serde-wasm-bindgen-0.4
  (package
    (name "rust-serde-wasm-bindgen")
    (version "0.4.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serde-wasm-bindgen" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1gr8hrr2zx9wqq02vh5lmsyhyaf0agvapf42glq1940drlqw1d73"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-js-sys" ,rust-js-sys-0.3)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-wasm-bindgen" ,rust-wasm-bindgen-0.2))))
    (home-page "https://github.com/RReverser/serde-wasm-bindgen")
    (synopsis "Native Serde adapter for wasm-bindgen")
    (description
     "This package provides Native Serde adapter for wasm-bindgen.")
    (license license:expat)))

(define-public rust-console-error-panic-hook-0.1
  (package
    (name "rust-console-error-panic-hook")
    (version "0.1.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "console_error_panic_hook" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1g5v8s0ndycc10mdn6igy914k645pgpcl8vjpz6nvxkhyirynsm0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-wasm-bindgen" ,rust-wasm-bindgen-0.2))))
    (home-page "https://github.com/rustwasm/console_error_panic_hook")
    (synopsis
     "panic hook for `wasm32-unknown-unknown` that logs panics to `console.error`")
    (description
     "This package provides a panic hook for `wasm32-unknown-unknown` that logs panics
to `console.error`.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-binding-macros-0.49
  (package
    (name "rust-binding-macros")
    (version "0.49.48")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "binding_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "01bcbxhd8jq76hiifpkhqjmmzslhiphl0z31zmal1872hryc7lqx"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-anyhow" ,rust-anyhow-1)
                       ("rust-console-error-panic-hook" ,rust-console-error-panic-hook-0.1)
                       ("rust-js-sys" ,rust-js-sys-0.3)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-wasm-bindgen" ,rust-serde-wasm-bindgen-0.4)
                       ("rust-swc" ,rust-swc-0.260)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-transforms" ,rust-swc-ecma-transforms-0.217)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-wasm-bindgen" ,rust-wasm-bindgen-0.2)
                       ("rust-wasm-bindgen-futures" ,rust-wasm-bindgen-futures-0.4))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "Macros to build customized bindings interface")
    (description
     "This package provides Macros to build customized bindings interface.")
    (license license:asl2.0)))

(define-public rust-swc-core-0.75
  (package
    (name "rust-swc-core")
    (version "0.75.48")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "swc_core" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lgbzlbm7k2jfq710an68j8pnbmwf7k3dz9zar1cdb81yzl7h403"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-binding-macros" ,rust-binding-macros-0.49)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-swc" ,rust-swc-0.260)
                       ("rust-swc-atoms" ,rust-swc-atoms-0.5)
                       ("rust-swc-bundler" ,rust-swc-bundler-0.213)
                       ("rust-swc-cached" ,rust-swc-cached-0.3)
                       ("rust-swc-common" ,rust-swc-common-0.31)
                       ("rust-swc-css-ast" ,rust-swc-css-ast-0.137)
                       ("rust-swc-css-codegen" ,rust-swc-css-codegen-0.147)
                       ("rust-swc-css-compat" ,rust-swc-css-compat-0.23)
                       ("rust-swc-css-minifier" ,rust-swc-css-minifier-0.112)
                       ("rust-swc-css-modules" ,rust-swc-css-modules-0.25)
                       ("rust-swc-css-parser" ,rust-swc-css-parser-0.146)
                       ("rust-swc-css-prefixer" ,rust-swc-css-prefixer-0.149)
                       ("rust-swc-css-utils" ,rust-swc-css-utils-0.134)
                       ("rust-swc-css-visit" ,rust-swc-css-visit-0.136)
                       ("rust-swc-ecma-ast" ,rust-swc-ecma-ast-0.103)
                       ("rust-swc-ecma-codegen" ,rust-swc-ecma-codegen-0.138)
                       ("rust-swc-ecma-loader" ,rust-swc-ecma-loader-0.43)
                       ("rust-swc-ecma-minifier" ,rust-swc-ecma-minifier-0.180)
                       ("rust-swc-ecma-parser" ,rust-swc-ecma-parser-0.133)
                       ("rust-swc-ecma-preset-env" ,rust-swc-ecma-preset-env-0.194)
                       ("rust-swc-ecma-quote-macros" ,rust-swc-ecma-quote-macros-0.44)
                       ("rust-swc-ecma-transforms-base" ,rust-swc-ecma-transforms-base-0.126)
                       ("rust-swc-ecma-transforms-compat" ,rust-swc-ecma-transforms-compat-0.152)
                       ("rust-swc-ecma-transforms-module" ,rust-swc-ecma-transforms-module-0.169)
                       ("rust-swc-ecma-transforms-optimization" ,rust-swc-ecma-transforms-optimization-0.186)
                       ("rust-swc-ecma-transforms-proposal" ,rust-swc-ecma-transforms-proposal-0.160)
                       ("rust-swc-ecma-transforms-react" ,rust-swc-ecma-transforms-react-0.172)
                       ("rust-swc-ecma-transforms-testing" ,rust-swc-ecma-transforms-testing-0.129)
                       ("rust-swc-ecma-transforms-typescript" ,rust-swc-ecma-transforms-typescript-0.176)
                       ("rust-swc-ecma-usage-analyzer" ,rust-swc-ecma-usage-analyzer-0.12)
                       ("rust-swc-ecma-utils" ,rust-swc-ecma-utils-0.116)
                       ("rust-swc-ecma-visit" ,rust-swc-ecma-visit-0.89)
                       ("rust-swc-node-base" ,rust-swc-node-base-0.5)
                       ("rust-swc-node-bundler" ,rust-swc-node-bundler-0.47)
                       ("rust-swc-nodejs-common" ,rust-swc-nodejs-common-0.0.5)
                       ("rust-swc-plugin" ,rust-swc-plugin-0.90)
                       ("rust-swc-plugin-macro" ,rust-swc-plugin-macro-0.9)
                       ("rust-swc-plugin-proxy" ,rust-swc-plugin-proxy-0.32)
                       ("rust-swc-plugin-runner" ,rust-swc-plugin-runner-0.94)
                       ("rust-swc-trace-macro" ,rust-swc-trace-macro-0.1)
                       ("rust-testing" ,rust-testing-0.33)
                       ("rust-vergen" ,rust-vergen-7))))
    (home-page "https://github.com/swc-project/swc.git")
    (synopsis "TBD")
    (description "This package provides TBD.")
    (license license:asl2.0)))

(define-public rust-markdown-1
  (package
    (name "rust-markdown")
    (version "1.0.0-alpha.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "markdown" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0v5nskcbhrq9iz5r7ias45412fxvvd2cw6i08664dbyq3phxli8w"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-log" ,rust-log-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-unicode-id" ,rust-unicode-id-0.3))
       #:cargo-development-inputs (("rust-criterion" ,rust-criterion-0.4)
                                   ("rust-env-logger" ,rust-env-logger-0.10)
                                   ("rust-pretty-assertions" ,rust-pretty-assertions-1)
                                   ("rust-swc-core" ,rust-swc-core-0.75))))
    (home-page "https://github.com/wooorm/markdown-rs")
    (synopsis
     "CommonMark compliant markdown parser in Rust with ASTs and extensions")
    (description
     "This package provides @code{CommonMark} compliant markdown parser in Rust with ASTs and extensions.")
    (license license:expat)))

(define-public rust-iso8601-duration-0.2
  (package
    (name "rust-iso8601-duration")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "iso8601-duration" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "04f5i2l9r4k44p822pqy59fmjxpk9yikgbbiq86s2g2x1bvdysm2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-chrono" ,rust-chrono-0.4)
                       ("rust-nom" ,rust-nom-7)
                       ("rust-serde" ,rust-serde-1))
       #:cargo-development-inputs (("rust-serde-json" ,rust-serde-json-1))))
    (home-page "https://github.com/PoiScript/iso8601-duration")
    (synopsis "Parse ISO8601 duration format")
    (description "This package provides Parse ISO8601 duration format.")
    (license license:expat)))

(define-public rust-fluent-pseudo-0.3
  (package
    (name "rust-fluent-pseudo")
    (version "0.3.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "fluent-pseudo" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1mp5rib3mzhiwbw9s3g0climzy4wxxp0angn5ycmspl0gdid6d81"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-regex" ,rust-regex-1))))
    (home-page "http://www.projectfluent.org")
    (synopsis
     "Pseudolocalization transformation API for use with Project Fluent API.")
    (description
     "This package provides Pseudolocalization transformation API for use with Project Fluent API.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-self-cell-0.10
  (package
    (name "rust-self-cell")
    (version "0.10.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "self_cell" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0pci3zh23b7dg6jmlxbn8k4plb7hcg5jprd1qiz0rp04p1ilskp1"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-self-cell" ,rust-self-cell-1))))
    (home-page "https://github.com/Voultapher/self_cell")
    (synopsis
     "Safe-to-use proc-macro-free self-referential structs in stable Rust")
    (description
     "This package provides Safe-to-use proc-macro-free self-referential structs in stable Rust.")
    (license license:asl2.0)))

(define-public rust-intl-pluralrules-7
  (package
    (name "rust-intl-pluralrules")
    (version "7.0.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "intl_pluralrules" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0wprd3h6h8nfj62d8xk71h178q7zfn3srxm787w4sawsqavsg3h7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-unic-langid" ,rust-unic-langid-0.9))))
    (home-page "https://github.com/zbraniecki/pluralrules")
    (synopsis "Unicode Plural Rules categorizer for numeric input")
    (description
     "This package provides Unicode Plural Rules categorizer for numeric input.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-type-map-0.5
  (package
    (name "rust-type-map")
    (version "0.5.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "type-map" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "17qaga12nkankr7hi2mv43f4lnc78hg480kz6j9zmy4g0h28ddny"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-rustc-hash" ,rust-rustc-hash-1))))
    (home-page "https://github.com/kardeiz/type-map")
    (synopsis "Provides a typemap container with FxHashMap")
    (description
     "This package provides a typemap container with @code{FxHashMap}.")
    (license (list license:expat license:asl2.0))))

(define-public rust-intl-memoizer-0.5
  (package
    (name "rust-intl-memoizer")
    (version "0.5.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "intl-memoizer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1nkvql7c7b76axv4g68di1p2m9bnxq1cbn6mlqcawf72zhhf08py"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-type-map" ,rust-type-map-0.5)
                       ("rust-unic-langid" ,rust-unic-langid-0.9))))
    (home-page "http://www.projectfluent.org")
    (synopsis "memoizer specifically tailored for storing lazy-initialized
intl formatters.")
    (description
     "This package provides a memoizer specifically tailored for storing
lazy-initialized intl formatters.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-fluent-syntax-0.11
  (package
    (name "rust-fluent-syntax")
    (version "0.11.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "fluent-syntax" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0gd3cdvsx9ymbb8hijcsc9wyf8h1pbcbpsafg4ldba56ji30qlra"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-thiserror" ,rust-thiserror-1))))
    (home-page "http://www.projectfluent.org")
    (synopsis "Parser/Serializer tools for Fluent Syntax.")
    (description
     "This package provides Parser/Serializer tools for Fluent Syntax.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-unic-langid-macros-impl-0.9
  (package
    (name "rust-unic-langid-macros-impl")
    (version "0.9.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "unic-langid-macros-impl" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0nsm0hky2sawgkwz511br06mkm3ba70rfc05jm0l54x3gciz9mqy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro-hack" ,rust-proc-macro-hack-0.5)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2)
                       ("rust-unic-langid-impl" ,rust-unic-langid-impl-0.9))))
    (home-page "https://github.com/zbraniecki/unic-locale")
    (synopsis "API for managing Unicode Language Identifiers")
    (description
     "This package provides API for managing Unicode Language Identifiers.")
    (license (list license:expat license:asl2.0))))

(define-public rust-unic-langid-macros-0.9
  (package
    (name "rust-unic-langid-macros")
    (version "0.9.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "unic-langid-macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0pi71r5474n7sdmyky7qpnia9rrr42q0d200l5lpag1d0hncv88d"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro-hack" ,rust-proc-macro-hack-0.5)
                       ("rust-tinystr" ,rust-tinystr-0.7)
                       ("rust-unic-langid-impl" ,rust-unic-langid-impl-0.9)
                       ("rust-unic-langid-macros-impl" ,rust-unic-langid-macros-impl-0.9))))
    (home-page "https://github.com/zbraniecki/unic-locale")
    (synopsis "API for managing Unicode Language Identifiers")
    (description
     "This package provides API for managing Unicode Language Identifiers.")
    (license (list license:expat license:asl2.0))))

(define-public rust-zerovec-derive-0.10
  (package
    (name "rust-zerovec-derive")
    (version "0.10.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "zerovec-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ik322dys6wnap5d3gcsn09azmssq466xryn5czfm13mn7gsdbvf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/unicode-org/icu4x")
    (synopsis "Custom derive for the zerovec crate")
    (description "This package provides Custom derive for the zerovec crate.")
    (license #f)))

(define-public rust-zerofrom-derive-0.1
  (package
    (name "rust-zerofrom-derive")
    (version "0.1.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "zerofrom-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "19b31rrs2ry1lrq5mpdqjzgg65va51fgvwghxnf6da3ycfiv99qf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2)
                       ("rust-synstructure" ,rust-synstructure-0.13))))
    (home-page "https://github.com/unicode-org/icu4x")
    (synopsis "Custom derive for the zerofrom crate")
    (description "This package provides Custom derive for the zerofrom crate.")
    (license #f)))

(define-public rust-zerofrom-0.1
  (package
    (name "rust-zerofrom")
    (version "0.1.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "zerofrom" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0mdbjd7vmbix2ynxbrbrrli47a5yrpfx05hi99wf1l4pwwf13v4i"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-zerofrom-derive" ,rust-zerofrom-derive-0.1))))
    (home-page "https://github.com/unicode-org/icu4x")
    (synopsis "ZeroFrom trait for constructing")
    (description
     "This package provides @code{ZeroFrom} trait for constructing.")
    (license #f)))

(define-public rust-yoke-derive-0.7
  (package
    (name "rust-yoke-derive")
    (version "0.7.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "yoke-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "15cvhkci2mchfffx3fmva84fpmp34dsmnbzibwfnzjqq3ds33k18"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2)
                       ("rust-synstructure" ,rust-synstructure-0.13))))
    (home-page "https://github.com/unicode-org/icu4x")
    (synopsis "Custom derive for the yoke crate")
    (description "This package provides Custom derive for the yoke crate.")
    (license #f)))

(define-public rust-yoke-0.7
  (package
    (name "rust-yoke")
    (version "0.7.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "yoke" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "198c4jkh6i3hxijia7mfa4cpnxg1iqym9bz364697c3rn0a16nvc"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1)
                       ("rust-stable-deref-trait" ,rust-stable-deref-trait-1)
                       ("rust-yoke-derive" ,rust-yoke-derive-0.7)
                       ("rust-zerofrom" ,rust-zerofrom-0.1))))
    (home-page "https://github.com/unicode-org/icu4x")
    (synopsis
     "Abstraction allowing borrowed data to be carried along with the backing data it borrows from")
    (description
     "This package provides Abstraction allowing borrowed data to be carried along with the backing data it
borrows from.")
    (license #f)))

(define-public rust-zerovec-0.10
  (package
    (name "rust-zerovec")
    (version "0.10.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "zerovec" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0yghix7n3fjfdppwghknzvx9v8cf826h2qal5nqvy8yzg4yqjaxa"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-databake" ,rust-databake-0.1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-twox-hash" ,rust-twox-hash-1)
                       ("rust-yoke" ,rust-yoke-0.7)
                       ("rust-zerofrom" ,rust-zerofrom-0.1)
                       ("rust-zerovec-derive" ,rust-zerovec-derive-0.10))))
    (home-page "https://github.com/unicode-org/icu4x")
    (synopsis "Zero-copy vector backed by a byte array")
    (description
     "This package provides Zero-copy vector backed by a byte array.")
    (license #f)))

(define-public rust-synstructure-0.13
  (package
    (name "rust-synstructure")
    (version "0.13.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "synstructure" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0wc9f002ia2zqcbj0q2id5x6n7g1zjqba7qkg2mr0qvvmdk7dby8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/mystor/synstructure")
    (synopsis "Helper methods and macros for custom derives")
    (description
     "This package provides Helper methods and macros for custom derives.")
    (license license:expat)))

(define-public rust-databake-derive-0.1
  (package
    (name "rust-databake-derive")
    (version "0.1.8")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "databake-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0yymbr1z93k7lg0pl5mw9mjhw8fpsfykg7bmkvmir9h1wmfjfy20"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2)
                       ("rust-synstructure" ,rust-synstructure-0.13))))
    (home-page "https://github.com/unicode-org/icu4x")
    (synopsis "Custom derive for the databake crate")
    (description "This package provides Custom derive for the databake crate.")
    (license #f)))

(define-public rust-databake-0.1
  (package
    (name "rust-databake")
    (version "0.1.8")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "databake" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0kb0lnhka1fklrii3qaj40zcrbclfn8fyvy0r1whd3yaxkxzn13a"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-databake-derive" ,rust-databake-derive-0.1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1))))
    (home-page "https://github.com/unicode-org/icu4x")
    (synopsis
     "Trait that lets structs represent themselves as (const) Rust expressions")
    (description
     "This package provides Trait that lets structs represent themselves as (const) Rust expressions.")
    (license #f)))

(define-public rust-tinystr-0.7
  (package
    (name "rust-tinystr")
    (version "0.7.6")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tinystr" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0bxqaw7z8r2kzngxlzlgvld1r6jbnwyylyvyjbv1q71rvgaga5wi"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-databake" ,rust-databake-0.1)
                       ("rust-displaydoc" ,rust-displaydoc-0.2)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-zerovec" ,rust-zerovec-0.10))))
    (home-page "https://github.com/unicode-org/icu4x")
    (synopsis "small ASCII-only bounded length string representation.")
    (description
     "This package provides a small ASCII-only bounded length string representation.")
    (license #f)))

(define-public rust-unic-langid-impl-0.9
  (package
    (name "rust-unic-langid-impl")
    (version "0.9.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "unic-langid-impl" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1rckyn5wqd5h8jxhbzlbbagr459zkzg822r4k5n30jaryv0j4m0a"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-tinystr" ,rust-tinystr-0.7))))
    (home-page "https://github.com/zbraniecki/unic-locale")
    (synopsis "API for managing Unicode Language Identifiers")
    (description
     "This package provides API for managing Unicode Language Identifiers.")
    (license (list license:expat license:asl2.0))))

(define-public rust-unic-langid-0.9
  (package
    (name "rust-unic-langid")
    (version "0.9.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "unic-langid" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0i2s024frmpfa68lzy8y8vnb1rz3m9v0ga13f7h2afx7f8g9vp93"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-unic-langid-impl" ,rust-unic-langid-impl-0.9)
                       ("rust-unic-langid-macros" ,rust-unic-langid-macros-0.9))))
    (home-page "https://github.com/zbraniecki/unic-locale")
    (synopsis "API for managing Unicode Language Identifiers")
    (description
     "This package provides API for managing Unicode Language Identifiers.")
    (license (list license:expat license:asl2.0))))

(define-public rust-fluent-langneg-0.13
  (package
    (name "rust-fluent-langneg")
    (version "0.13.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "fluent-langneg" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "152yxplc11vmxkslvmaqak9x86xnavnhdqyhrh38ym37jscd0jic"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-unic-langid" ,rust-unic-langid-0.9))))
    (home-page "http://projectfluent.org/")
    (synopsis "library for language and locale negotiation.")
    (description
     "This package provides a library for language and locale negotiation.")
    (license license:asl2.0)))

(define-public rust-fluent-bundle-0.15
  (package
    (name "rust-fluent-bundle")
    (version "0.15.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "fluent-bundle" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "14zl0cjn361is69pb1zry4k2zzh5nzsfv0iz05wccl00x0ga5q3z"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-fluent-langneg" ,rust-fluent-langneg-0.13)
                       ("rust-fluent-syntax" ,rust-fluent-syntax-0.11)
                       ("rust-intl-memoizer" ,rust-intl-memoizer-0.5)
                       ("rust-intl-pluralrules" ,rust-intl-pluralrules-7)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-self-cell" ,rust-self-cell-0.10)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-unic-langid" ,rust-unic-langid-0.9))))
    (home-page "http://www.projectfluent.org")
    (synopsis
     "localization system designed to unleash the entire expressive power of
natural language translations.")
    (description
     "This package provides a localization system designed to unleash the entire
expressive power of natural language translations.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-fluent-0.16
  (package
    (name "rust-fluent")
    (version "0.16.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "fluent" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0njmdpwz52yjzyp55iik9k6vrixqiy7190d98pk0rgdy0x3n6x5v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-fluent-bundle" ,rust-fluent-bundle-0.15)
                       ("rust-fluent-pseudo" ,rust-fluent-pseudo-0.3)
                       ("rust-unic-langid" ,rust-unic-langid-0.9))))
    (home-page "http://www.projectfluent.org")
    (synopsis
     "localization system designed to unleash the entire expressive power of
natural language translations.")
    (description
     "This package provides a localization system designed to unleash the entire
expressive power of natural language translations.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-uwl-0.6
  (package
    (name "rust-uwl")
    (version "0.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "uwl" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1l0spdpn879wpf440x4cdsbz5dilp5ihfsxsqkn2dmkhrbh07gzl"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/acdenisSK/uwl")
    (synopsis "management stream for bytes and characters")
    (description
     "This package provides a management stream for bytes and characters.")
    (license (list license:expat license:asl2.0))))

(define-public rust-typesize-derive-0.1
  (package
    (name "rust-typesize-derive")
    (version "0.1.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typesize-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1a0ypva9lwmyfgvng2iq87j2gw2ishhm2jbysmmnh9yclk18hplh"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/GnomedDev/typesize")
    (synopsis "Internal proc-macro crate for typesize")
    (description
     "This package provides Internal proc-macro crate for typesize.")
    (license license:expat)))

(define-public rust-nonmax-0.5
  (package
    (name "rust-nonmax")
    (version "0.5.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "nonmax" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lfvyfz4falgmc9g1cbfi2wkys9wka2nfmdyga87zikf636ml2k1"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/LPGhatguy/nonmax")
    (synopsis "Numeric types that cannot hold maximum values")
    (description
     "This package provides Numeric types that cannot hold maximum values.")
    (license (list license:expat license:asl2.0))))

(define-public rust-gat-lending-iterator-0.1
  (package
    (name "rust-gat-lending-iterator")
    (version "0.1.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "gat-lending-iterator" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1agx13403m08gk13sv8qhy9m4na97bm3lgpa1m0bdmdayawpncj2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/Crazytieguy/gat-lending-iterator")
    (synopsis
     "library for iterators who's items can [mutably] reference the iterator.")
    (description
     "This package provides a library for iterators who's items can [mutably]
reference the iterator.")
    (license license:expat)))

(define-public rust-extract-map-0.1
  (package
    (name "rust-extract-map")
    (version "0.1.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "extract_map" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0s695n5yzs7wwyvaphkkscr8lfv7h94xsczg49a9qa37nnd51xxb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-gat-lending-iterator" ,rust-gat-lending-iterator-0.1)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/GnomedDev/extract_map")
    (synopsis
     "HashMap for memory efficent storage of value types which contain their own keys.")
    (description
     "This package provides a @code{HashMap} for memory efficent storage of value
types which contain their own keys.")
    (license license:expat)))

(define-public rust-typesize-0.1
  (package
    (name "rust-typesize")
    (version "0.1.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typesize" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03zf3k54hbn0mvcx1axv5b5crvlb435h9rwr7vv7dg09qx14hw7b"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-extract-map" ,rust-extract-map-0.1)
                       ("rust-halfbrown" ,rust-halfbrown-0.2)
                       ("rust-hashbrown" ,rust-hashbrown-0.14)
                       ("rust-mini-moka" ,rust-mini-moka-0.10)
                       ("rust-nonmax" ,rust-nonmax-0.5)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-secrecy" ,rust-secrecy-0.8)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-simd-json" ,rust-simd-json-0.13)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-typesize-derive" ,rust-typesize-derive-0.1)
                       ("rust-url" ,rust-url-2))))
    (home-page "https://github.com/GnomedDev/typesize")
    (synopsis
     "library to fetch an accurate estimate of the total memory usage of a value.")
    (description
     "This package provides a library to fetch an accurate estimate of the total
memory usage of a value.")
    (license license:expat)))

(define-public rust-typemap-rev-0.3
  (package
    (name "rust-typemap-rev")
    (version "0.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typemap_rev" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "161935l8j5jxzjz64g4z21z3x7aj9ljhadjwdbqilf2p2868pc3l"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/bdashore3/typemap_rev")
    (synopsis "hashmap, but stores types as keys")
    (description "This package provides a hashmap, but stores types as keys.")
    (license license:isc)))

(define-public rust-tungstenite-0.21
  (package
    (name "rust-tungstenite")
    (version "0.21.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tungstenite" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1qaphb5kgwgid19p64grhv2b9kxy7f1059yy92l9kwrlx90sdwcy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-byteorder" ,rust-byteorder-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-data-encoding" ,rust-data-encoding-2)
                       ("rust-http" ,rust-http-1)
                       ("rust-httparse" ,rust-httparse-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-rustls" ,rust-rustls-0.22)
                       ("rust-rustls-native-certs" ,rust-rustls-native-certs-0.7)
                       ("rust-rustls-pki-types" ,rust-rustls-pki-types-1)
                       ("rust-sha1" ,rust-sha1-0.10)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-url" ,rust-url-2)
                       ("rust-utf-8" ,rust-utf-8-0.7)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.26))))
    (home-page "https://github.com/snapview/tungstenite-rs")
    (synopsis "Lightweight stream-based WebSocket implementation")
    (description
     "This package provides Lightweight stream-based @code{WebSocket} implementation.")
    (license (list license:expat license:asl2.0))))

(define-public rust-tokio-tungstenite-0.21
  (package
    (name "rust-tokio-tungstenite")
    (version "0.21.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tokio-tungstenite" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0f5wj0crsx74rlll97lhw0wk6y12nhdnqvmnjx002hjn08fmcfy8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-rustls" ,rust-rustls-0.22)
                       ("rust-rustls-native-certs" ,rust-rustls-native-certs-0.7)
                       ("rust-rustls-pki-types" ,rust-rustls-pki-types-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-native-tls" ,rust-tokio-native-tls-0.3)
                       ("rust-tokio-rustls" ,rust-tokio-rustls-0.25)
                       ("rust-tungstenite" ,rust-tungstenite-0.21)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.26))))
    (home-page "https://github.com/snapview/tokio-tungstenite")
    (synopsis
     "Tokio binding for Tungstenite, the Lightweight stream-based WebSocket implementation")
    (description
     "This package provides Tokio binding for Tungstenite, the Lightweight stream-based @code{WebSocket}
implementation.")
    (license license:expat)))

(define-public rust-typewit-proc-macros-1
  (package
    (name "rust-typewit-proc-macros")
    (version "1.8.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typewit_proc_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1mlkh4mhbn4b7xg9640blk74bm5ddaa44ihvl0sljw1w5gm86sp3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/rodrimati1992/typewit/")
    (synopsis "implementation detail of typewit")
    (description "This package provides implementation detail of typewit.")
    (license license:zlib)))

(define-public rust-typewit-1
  (package
    (name "rust-typewit")
    (version "1.9.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typewit" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "093fgb1q5n48vr4nj3hggbhfi6jzab5048scs6jz1ynalgk9myy6"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-typewit-proc-macros" ,rust-typewit-proc-macros-1))))
    (home-page "https://github.com/rodrimati1992/typewit/")
    (synopsis
     "type-witness-based abstractions, mostly for emulating polymorphism in const fns")
    (description
     "This package provides type-witness-based abstractions, mostly for emulating polymorphism in const fns.")
    (license license:zlib)))

(define-public rust-const-panic-proc-macros-0.2
  (package
    (name "rust-const-panic-proc-macros")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "const_panic_proc_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1addx3a8vi02cdak3ygrqivv02jj73251h85x49aic78yznrhlrr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1)
                       ("rust-unicode-xid" ,rust-unicode-xid-0.2))))
    (home-page "https://github.com/rodrimati1992/const_panic/")
    (synopsis "Implementation detail of the `const_panic` crate")
    (description
     "This package provides Implementation detail of the `const_panic` crate.")
    (license license:zlib)))

(define-public rust-const-panic-0.2
  (package
    (name "rust-const-panic")
    (version "0.2.8")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "const_panic" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16w72mnzjqgwfhlq8cqm6xhd2n6lc1wan08987izv1pcxhwz4lb0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-const-panic-proc-macros" ,rust-const-panic-proc-macros-0.2)
                       ("rust-typewit" ,rust-typewit-1))))
    (home-page "https://github.com/rodrimati1992/const_panic/")
    (synopsis "const panic with formatting")
    (description "This package provides const panic with formatting.")
    (license license:zlib)))

(define-public rust-as-derive-utils-0.11
  (package
    (name "rust-as-derive-utils")
    (version "0.11.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "as_derive_utils" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1i2kwzxdhydicj9bqscz5w73nmx612yi3ha137qlr900b5j9cg7z"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-aho-corasick" ,rust-aho-corasick-0.7)
                       ("rust-bitflags" ,rust-bitflags-1)
                       ("rust-core-extensions" ,rust-core-extensions-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-ron" ,rust-ron-0.7)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-derive" ,rust-serde-derive-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/rodrimati1992/abi_stable_crates/")
    (synopsis "private derive utilities used by abi_stable and structural")
    (description
     "This package provides private derive utilities used by abi_stable and structural.")
    (license (list license:expat license:asl2.0))))

(define-public rust-abi-stable-shared-0.11
  (package
    (name "rust-abi-stable-shared")
    (version "0.11.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "abi_stable_shared" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0qrbmlypvxx3zij1c6w6yykpp5pjcfx9qr2d9lzyc8y1i1vdzddj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-core-extensions" ,rust-core-extensions-1))))
    (home-page "https://github.com/rodrimati1992/abi_stable_crates/")
    (synopsis "Implementation detail of abi_stable")
    (description "This package provides Implementation detail of abi_stable.")
    (license (list license:expat license:asl2.0))))

(define-public rust-abi-stable-derive-0.11
  (package
    (name "rust-abi-stable-derive")
    (version "0.11.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "abi_stable_derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16780mmr2hwx8ajcq59nhvq3krv5i8r7mg41x08fx907nil885yp"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-abi-stable-shared" ,rust-abi-stable-shared-0.11)
                       ("rust-as-derive-utils" ,rust-as-derive-utils-0.11)
                       ("rust-core-extensions" ,rust-core-extensions-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-rustc-version" ,rust-rustc-version-0.4)
                       ("rust-syn" ,rust-syn-1)
                       ("rust-typed-arena" ,rust-typed-arena-2))))
    (home-page "https://github.com/rodrimati1992/abi_stable_crates/")
    (synopsis "Implementation detail of abi_stable")
    (description "This package provides Implementation detail of abi_stable.")
    (license (list license:expat license:asl2.0))))

(define-public rust-abi-stable-0.11
  (package
    (name "rust-abi-stable")
    (version "0.11.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "abi_stable" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0if428pq8ly97zi6q1842nak977rwxnj17650i8gwpxh7qnm3mk9"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-abi-stable-derive" ,rust-abi-stable-derive-0.11)
                       ("rust-abi-stable-shared" ,rust-abi-stable-shared-0.11)
                       ("rust-const-panic" ,rust-const-panic-0.2)
                       ("rust-core-extensions" ,rust-core-extensions-1)
                       ("rust-crossbeam-channel" ,rust-crossbeam-channel-0.5)
                       ("rust-generational-arena" ,rust-generational-arena-0.2)
                       ("rust-libloading" ,rust-libloading-0.7)
                       ("rust-lock-api" ,rust-lock-api-0.4)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-paste" ,rust-paste-1)
                       ("rust-repr-offset" ,rust-repr-offset-0.2)
                       ("rust-rustc-version" ,rust-rustc-version-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-derive" ,rust-serde-derive-1)
                       ("rust-serde-json" ,rust-serde-json-1))))
    (home-page "https://github.com/rodrimati1992/abi_stable_crates/")
    (synopsis
     "For doing Rust-to-Rust ffi,writing libraries loaded at program startup")
    (description
     "This package provides For doing Rust-to-Rust ffi,writing libraries loaded at program startup.")
    (license (list license:expat license:asl2.0))))

(define-public rust-value-trait-0.8
  (package
    (name "rust-value-trait")
    (version "0.8.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "value-trait" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1d3yl8g6xzp4ah4r7nrqrb1vxy9vgp53z80vy9ypjxz6q6cdpn6s"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-abi-stable" ,rust-abi-stable-0.11)
                       ("rust-float-cmp" ,rust-float-cmp-0.9)
                       ("rust-halfbrown" ,rust-halfbrown-0.2)
                       ("rust-hashbrown" ,rust-hashbrown-0.14)
                       ("rust-itoa" ,rust-itoa-1)
                       ("rust-ryu" ,rust-ryu-1))))
    (home-page "https://github.com/simd-lite/value-trait")
    (synopsis "Traits to deal with JSONesque values")
    (description "This package provides Traits to deal with JSONesque values.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-x86-0.47
  (package
    (name "rust-x86")
    (version "0.47.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "x86" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jlddyczw168mcy4a6m3nbl203rxli2vr5gcmf57s0adqf6bxdam"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bit-field" ,rust-bit-field-0.10)
                       ("rust-bitflags" ,rust-bitflags-1)
                       ("rust-csv" ,rust-csv-1)
                       ("rust-phf" ,rust-phf-0.9)
                       ("rust-phf-codegen" ,rust-phf-codegen-0.9)
                       ("rust-raw-cpuid" ,rust-raw-cpuid-10)
                       ("rust-serde-json" ,rust-serde-json-1))))
    (home-page "https://github.com/gz/rust-x86")
    (synopsis
     "Library to program x86 (amd64) hardware. Contains x86 specific data structure descriptions, data-tables, as well as convenience function to call assembly instructions typically not exposed in higher level languages")
    (description
     "This package provides Library to program x86 (amd64) hardware.  Contains x86 specific data structure
descriptions, data-tables, as well as convenience function to call assembly
instructions typically not exposed in higher level languages.")
    (license license:expat)))

(define-public rust-libc-0.1
  (package
    (name "rust-libc")
    (version "0.1.12")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "libc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "08k14zb7bw25avmaj227calcdglb4ac394kklr9nv175fp7p0ap3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/rust-lang/libc")
    (synopsis "Raw FFI bindings to platform libraries like libc.")
    (description
     "This package provides Raw FFI bindings to platform libraries like libc.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mmap-0.1
  (package
    (name "rust-mmap")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mmap" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "08xqhvr4l3rf1fkz2w4cwz3z5wd0m1jab1d34sxd4v80lr459j0b"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-libc" ,rust-libc-0.1)
                       ("rust-tempdir" ,rust-tempdir-0.3))))
    (home-page "https://github.com/rbranson/rust-mmap")
    (synopsis "library for dealing with memory-mapped I/O")
    (description
     "This package provides a library for dealing with memory-mapped I/O.")
    (license license:expat)))

(define-public rust-perfcnt-0.8
  (package
    (name "rust-perfcnt")
    (version "0.8.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "perfcnt" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "008mrdd8zjk54qg8xh8crk9is98sxv2c0kk2v25nzjkhaaazv8ab"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-mmap" ,rust-mmap-0.1)
                       ("rust-nom" ,rust-nom-4)
                       ("rust-x86" ,rust-x86-0.47))))
    (home-page "https://github.com/gz/rust-perfcnt")
    (synopsis
     "Library to configure and read hardware performance counters in rust")
    (description
     "This package provides Library to configure and read hardware performance counters in rust.")
    (license license:expat)))

(define-public rust-halfbrown-0.2
  (package
    (name "rust-halfbrown")
    (version "0.2.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "halfbrown" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0vzg46039pd730vc2hdhl09h86j4cd007awwlrf8l407hqd6d245"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-hashbrown" ,rust-hashbrown-0.14)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/Licenser/halfbrown")
    (synopsis
     "Multi backend HashMap for higher performance on different key space sizes")
    (description
     "This package provides Multi backend @code{HashMap} for higher performance on different key space
sizes.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-alloc-counter-macro-0.0.2
  (package
    (name "rust-alloc-counter-macro")
    (version "0.0.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "alloc_counter_macro" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0nifqalryavmrdlkyv7cznp8yfjj16x0bjqzvjndw0fxk8gzhlhs"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "")
    (synopsis "The #[no_alloc] macro for the alloc_counter crate")
    (description
     "This package provides The #[no_alloc] macro for the alloc_counter crate.")
    (license (list license:expat license:asl2.0))))

(define-public rust-alloc-counter-0.0.4
  (package
    (name "rust-alloc-counter")
    (version "0.0.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "alloc_counter" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1agxzprqi37bcy9hh3clbsl3n0awbb34vrlv4rp5afib8w53m31s"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-alloc-counter-macro" ,rust-alloc-counter-macro-0.0.2)
                       ("rust-pin-utils" ,rust-pin-utils-0.1))))
    (home-page "https://gitlab.com/sio4/code/alloc-counter")
    (synopsis
     "Count allocations, reallocations, deallocations. Allow, deny, or forbid allocations on an expression or function basis")
    (description
     "This package provides Count allocations, reallocations, deallocations.  Allow, deny, or forbid
allocations on an expression or function basis.")
    (license (list license:expat license:asl2.0))))

(define-public rust-simd-json-0.13
  (package
    (name "rust-simd-json")
    (version "0.13.10")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "simd-json" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1yj3h676qd8jk80xp4vxhm0gwbbqmrik51by161s0blh7l5l632p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-alloc-counter" ,rust-alloc-counter-0.0.4)
                       ("rust-beef" ,rust-beef-0.5)
                       ("rust-colored" ,rust-colored-2)
                       ("rust-getopts" ,rust-getopts-0.2)
                       ("rust-getrandom" ,rust-getrandom-0.2)
                       ("rust-halfbrown" ,rust-halfbrown-0.2)
                       ("rust-jemallocator" ,rust-jemallocator-0.5)
                       ("rust-lexical-core" ,rust-lexical-core-0.8)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-perfcnt" ,rust-perfcnt-0.8)
                       ("rust-ref-cast" ,rust-ref-cast-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-simdutf8" ,rust-simdutf8-0.1)
                       ("rust-value-trait" ,rust-value-trait-0.8))))
    (home-page "https://github.com/simd-lite/simd-json")
    (synopsis "High performance JSON parser based on a port of simdjson")
    (description
     "This package provides High performance JSON parser based on a port of simdjson.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-serenity-voice-model-0.2
  (package
    (name "rust-serenity-voice-model")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serenity-voice-model" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0sz0h345as4ypmk2z7yarab6fzmbb87hdldk66rwh1sx2pv84djr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-2)
                       ("rust-num-traits" ,rust-num-traits-0.2)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serde-repr" ,rust-serde-repr-0.1))))
    (home-page "https://github.com/serenity-rs/serenity")
    (synopsis
     "Rust library for (de)serializing Discord Voice API gateway messages.")
    (description
     "This package provides a Rust library for (de)serializing Discord Voice API
gateway messages.")
    (license license:isc)))

(define-public rust-serde-derive-1
  (package
    (name "rust-serde-derive")
    (version "1.0.204")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serde_derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "08p25262mbmhsr2cg0508d5b1wvljj956rvpg0v3qgg6gc8pxkg0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://serde.rs")
    (synopsis "Macros 1.1 implementation of #[derive(Serialize, Deserialize)]")
    (description
     "This package provides Macros 1.1 implementation of #[derive(Serialize, Deserialize)].")
    (license (list license:expat license:asl2.0))))

(define-public rust-serde-cow-0.1
  (package
    (name "rust-serde-cow")
    (version "0.1.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serde_cow" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1nmplkb2bvk7iqijh01856b89h4783inajxmb8jxxgwnf7nbnyqy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/GnomedDev/serde_cow")
    (synopsis "library with more efficent serde deserializations for Cow.")
    (description
     "This package provides a library with more efficent serde deserializations for
Cow.")
    (license license:expat)))

(define-public rust-secrecy-0.8
  (package
    (name "rust-secrecy")
    (version "0.8.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "secrecy" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "07p9h2bpkkg61f1fzzdqqbf74kwv1gg095r1cdmjzzbcl17cblcv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytes" ,rust-bytes-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://github.com/iqlusioninc/crates/")
    (synopsis
     "Wrapper types and traits for secret management which help ensure
they aren't accidentally copied, logged, or otherwise exposed
(as much as possible), and also ensure secrets are securely wiped
from memory when dropped.")
    (description
     "This package provides Wrapper types and traits for secret management which help ensure they aren't
accidentally copied, logged, or otherwise exposed (as much as possible), and
also ensure secrets are securely wiped from memory when dropped.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-tagptr-0.2
  (package
    (name "rust-tagptr")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tagptr" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "05r4mwvlsclx1ayj65hpzjv3dn4wpi8j4xm695vydccf9k7r683v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/oliver-giersch/tagptr.git")
    (synopsis "Strongly typed atomic and non-atomic tagged pointers")
    (description
     "This package provides Strongly typed atomic and non-atomic tagged pointers.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mini-moka-0.10
  (package
    (name "rust-mini-moka")
    (version "0.10.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mini-moka" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "00yqhslppnrl2a54rrrp03xb65d2knbb1s5yvs3g6qgjcnmxy9f3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-crossbeam-channel" ,rust-crossbeam-channel-0.5)
                       ("rust-crossbeam-utils" ,rust-crossbeam-utils-0.8)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-skeptic" ,rust-skeptic-0.13)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-tagptr" ,rust-tagptr-0.2)
                       ("rust-triomphe" ,rust-triomphe-0.1))))
    (home-page "https://github.com/moka-rs/mini-moka")
    (synopsis "lighter edition of Moka, a fast and concurrent cache library")
    (description
     "This package provides a lighter edition of Moka, a fast and concurrent cache
library.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mime-guess-2
  (package
    (name "rust-mime-guess")
    (version "2.0.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mime_guess" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03jmg3yx6j39mg0kayf7w4a886dl3j15y8zs119zw01ccy74zi7p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-mime" ,rust-mime-0.3)
                       ("rust-unicase" ,rust-unicase-2)
                       ("rust-unicase" ,rust-unicase-2))))
    (home-page "https://github.com/abonander/mime_guess")
    (synopsis
     "simple crate for detection of a file's MIME type by its extension.")
    (description
     "This package provides a simple crate for detection of a file's MIME type by its
extension.")
    (license license:expat)))

(define-public rust-merlin-3
  (package
    (name "rust-merlin")
    (version "3.0.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "merlin" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0z9rh9jlpcs0i0cijbs6pcq26gl4qwz05y7zbnv7h2gwk4kqxhsq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-byteorder" ,rust-byteorder-1)
                       ("rust-hex" ,rust-hex-0.3)
                       ("rust-keccak" ,rust-keccak-0.1)
                       ("rust-rand-core" ,rust-rand-core-0.6)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://docs.rs/merlin")
    (synopsis
     "Composable proof transcripts for public-coin arguments of knowledge")
    (description
     "This package provides Composable proof transcripts for public-coin arguments of knowledge.")
    (license license:expat)))

(define-public rust-ed25519-2
  (package
    (name "rust-ed25519")
    (version "2.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "ed25519" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lydzdf26zbn82g7xfczcac9d7mzm3qgx934ijjrd5hjpjx32m8i"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-pkcs8" ,rust-pkcs8-0.10)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-bytes" ,rust-serde-bytes-0.11)
                       ("rust-signature" ,rust-signature-2)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://github.com/RustCrypto/signatures/tree/master/ed25519")
    (synopsis
     "Edwards Digital Signature Algorithm (EdDSA) over Curve25519 (as specified in RFC 8032)
support library providing signature type definitions and PKCS#8 private key
decoding/encoding support")
    (description
     "This package provides Edwards Digital Signature Algorithm (@code{EdDSA}) over Curve25519 (as specified
in RFC 8032) support library providing signature type definitions and PKCS#8
private key decoding/encoding support.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-fiat-crypto-0.2
  (package
    (name "rust-fiat-crypto")
    (version "0.2.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "fiat-crypto" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "07c1vknddv3ak7w89n85ik0g34nzzpms6yb845vrjnv9m4csbpi8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/mit-plv/fiat-crypto")
    (synopsis "Fiat-crypto generated Rust")
    (description "This package provides Fiat-crypto generated Rust.")
    (license (list license:expat license:asl2.0))))

(define-public rust-curve25519-dalek-derive-0.1
  (package
    (name "rust-curve25519-dalek-derive")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "curve25519-dalek-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1cry71xxrr0mcy5my3fb502cwfxy6822k4pm19cwrilrg7hq4s7l"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/dalek-cryptography/curve25519-dalek")
    (synopsis "curve25519-dalek Derives")
    (description "This package provides curve25519-dalek Derives.")
    (license (list license:expat license:asl2.0))))

(define-public rust-curve25519-dalek-4
  (package
    (name "rust-curve25519-dalek")
    (version "4.1.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "curve25519-dalek" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1gmjb9dsknrr8lypmhkyjd67p1arb8mbfamlwxm7vph38my8pywp"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-cpufeatures" ,rust-cpufeatures-0.2)
                       ("rust-curve25519-dalek-derive" ,rust-curve25519-dalek-derive-0.1)
                       ("rust-digest" ,rust-digest-0.10)
                       ("rust-ff" ,rust-ff-0.13)
                       ("rust-fiat-crypto" ,rust-fiat-crypto-0.2)
                       ("rust-group" ,rust-group-0.13)
                       ("rust-rand-core" ,rust-rand-core-0.6)
                       ("rust-rustc-version" ,rust-rustc-version-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-subtle" ,rust-subtle-2)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://github.com/dalek-cryptography/curve25519-dalek")
    (synopsis
     "pure-Rust implementation of group operations on ristretto255 and Curve25519")
    (description
     "This package provides a pure-Rust implementation of group operations on
ristretto255 and Curve25519.")
    (license license:bsd-3)))

(define-public rust-ed25519-dalek-2
  (package
    (name "rust-ed25519-dalek")
    (version "2.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "ed25519-dalek" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0w88cafwglg9hjizldbmlza0ns3hls81zk1bcih3m5m3h67algaa"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-curve25519-dalek" ,rust-curve25519-dalek-4)
                       ("rust-ed25519" ,rust-ed25519-2)
                       ("rust-merlin" ,rust-merlin-3)
                       ("rust-rand-core" ,rust-rand-core-0.6)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-signature" ,rust-signature-2)
                       ("rust-subtle" ,rust-subtle-2)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://github.com/dalek-cryptography/curve25519-dalek")
    (synopsis
     "Fast and efficient ed25519 EdDSA key generations, signing, and verification in pure Rust")
    (description
     "This package provides Fast and efficient ed25519 @code{EdDSA} key generations, signing, and
verification in pure Rust.")
    (license license:bsd-3)))

(define-public rust-command-attr-0.5
  (package
    (name "rust-command-attr")
    (version "0.5.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "command_attr" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0cbnvjdyhfc47rim5sgc1zkrvd0287wd0wng7y70vwz6kxz8vnl8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "")
    (synopsis
     "Procedural macros for command creation for the Serenity library")
    (description
     "This package provides Procedural macros for command creation for the Serenity library.")
    (license license:isc)))

(define-public rust-bytes-1
  (package
    (name "rust-bytes")
    (version "1.7.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "bytes" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jgss90klcbgqsafnjlzymy8gdzw785z7qsf6sp2p0a3bhfvx8pw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/tokio-rs/bytes")
    (synopsis "Types and traits for working with bytes")
    (description
     "This package provides Types and traits for working with bytes.")
    (license license:expat)))

(define-public rust-bitflags-2
  (package
    (name "rust-bitflags")
    (version "2.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "bitflags" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1pkidwzn3hnxlsl8zizh0bncgbjnw7c41cx7bby26ncbzmiznj5h"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arbitrary" ,rust-arbitrary-1)
                       ("rust-bytemuck" ,rust-bytemuck-1)
                       ("rust-compiler-builtins" ,rust-compiler-builtins-0.1)
                       ("rust-rustc-std-workspace-core" ,rust-rustc-std-workspace-core-1)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/bitflags/bitflags")
    (synopsis "macro to generate structures which behave like bitflags.")
    (description
     "This package provides a macro to generate structures which behave like bitflags.")
    (license (list license:expat license:asl2.0))))

(define-public rust-base64-0.22
  (package
    (name "rust-base64")
    (version "0.22.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "base64" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1imqzgh7bxcikp5vx3shqvw9j09g9ly0xr0jma0q66i52r7jbcvj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/marshallpierce/rust-base64")
    (synopsis "encodes and decodes base64 as bytes or utf8")
    (description
     "This package provides encodes and decodes base64 as bytes or utf8.")
    (license (list license:expat license:asl2.0))))

(define-public rust-serenity-0.12
  (package
    (name "rust-serenity")
    (version "0.12.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serenity" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1b1lblimgn4w9nc78w2gl6scrywvi0sxkcdcppssil4jcl8082l8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-base64" ,rust-base64-0.22)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-command-attr" ,rust-command-attr-0.5)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-ed25519-dalek" ,rust-ed25519-dalek-2)
                       ("rust-flate2" ,rust-flate2-1)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-fxhash" ,rust-fxhash-0.2)
                       ("rust-levenshtein" ,rust-levenshtein-1)
                       ("rust-mime-guess" ,rust-mime-guess-2)
                       ("rust-mini-moka" ,rust-mini-moka-0.10)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-percent-encoding" ,rust-percent-encoding-2)
                       ("rust-reqwest" ,rust-reqwest-0.11)
                       ("rust-secrecy" ,rust-secrecy-0.8)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-cow" ,rust-serde-cow-0.1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serenity-voice-model" ,rust-serenity-voice-model-0.2)
                       ("rust-simd-json" ,rust-simd-json-0.13)
                       ("rust-static-assertions" ,rust-static-assertions-1)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-tungstenite" ,rust-tokio-tungstenite-0.21)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-typemap-rev" ,rust-typemap-rev-0.3)
                       ("rust-typesize" ,rust-typesize-0.1)
                       ("rust-url" ,rust-url-2)
                       ("rust-uwl" ,rust-uwl-0.6))))
    (home-page "https://github.com/serenity-rs/serenity")
    (synopsis "Rust library for the Discord API.")
    (description "This package provides a Rust library for the Discord API.")
    (license license:isc)))

(define-public rust-poise-macros-0.6
  (package
    (name "rust-poise-macros")
    (version "0.6.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "poise_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1spz3rxx6ziayvdl0m0g8vj2pwbp65kcgsixrlaq7rv1r4iw38lg"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-darling" ,rust-darling-0.20)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/serenity-rs/poise/")
    (synopsis "Internal macro implementation crate of poise")
    (description
     "This package provides Internal macro implementation crate of poise.")
    (license license:expat)))

(define-public rust-poise-0.6
  (package
    (name "rust-poise")
    (version "0.6.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "poise" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "19h9z88l4aqijgwdp2vr42hm832p69jf9g2aflryz41mbsjda68q"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-derivative" ,rust-derivative-2)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-poise-macros" ,rust-poise-macros-0.6)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-serenity" ,rust-serenity-0.12)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tracing" ,rust-tracing-0.1))
       #:cargo-development-inputs (("rust-env-logger" ,rust-env-logger-0.10)
                                   ("rust-fluent" ,rust-fluent-0.16)
                                   ("rust-fluent-syntax" ,rust-fluent-syntax-0.11)
                                   ("rust-futures" ,rust-futures-0.3)
                                   ("rust-intl-memoizer" ,rust-intl-memoizer-0.5)
                                   ("rust-rand" ,rust-rand-0.8)
                                   ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/serenity-rs/poise/")
    (synopsis "Discord bot framework for serenity")
    (description "This package provides a Discord bot framework for serenity.")
    (license license:expat)))

(define-public rust-trust-dns-rustls-0.20
  (package
    (name "rust-trust-dns-rustls")
    (version "0.20.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "trust-dns-rustls" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ygfcp65xrjgsa3mkyk54fq1n34wis866bh3lx3jy6hxfgz3a4dr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-futures-channel" ,rust-futures-channel-0.3)
                       ("rust-futures-io" ,rust-futures-io-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-rustls" ,rust-rustls-0.19)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-rustls" ,rust-tokio-rustls-0.22)
                       ("rust-trust-dns-proto" ,rust-trust-dns-proto-0.20)
                       ("rust-webpki" ,rust-webpki-0.21))))
    (home-page "http://www.trust-dns.org/index.html")
    (synopsis
     "Trust-DNS is a safe and secure DNS library. This is an extension for the Trust-DNS client to use rustls for TLS.")
    (description
     "This package provides Trust-DNS is a safe and secure DNS library.  This is an extension for the
Trust-DNS client to use rustls for TLS.")
    (license (list license:expat license:asl2.0))))

(define-public rust-trust-dns-proto-0.20
  (package
    (name "rust-trust-dns-proto")
    (version "0.20.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "trust-dns-proto" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0cafw8m2488xlr251b0khf6h2d7g4ix0s164j33838dnzvlx956a"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-backtrace" ,rust-backtrace-0.3)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-data-encoding" ,rust-data-encoding-2)
                       ("rust-enum-as-inner" ,rust-enum-as-inner-0.3)
                       ("rust-futures-channel" ,rust-futures-channel-0.3)
                       ("rust-futures-io" ,rust-futures-io-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-idna" ,rust-idna-0.2)
                       ("rust-ipnet" ,rust-ipnet-2)
                       ("rust-js-sys" ,rust-js-sys-0.3)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-openssl" ,rust-openssl-0.10)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-ring" ,rust-ring-0.16)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-socket2" ,rust-socket2-0.3)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tinyvec" ,rust-tinyvec-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-url" ,rust-url-2)
                       ("rust-wasm-bindgen" ,rust-wasm-bindgen-0.2))))
    (home-page "https://trust-dns.org/")
    (synopsis
     "Trust-DNS is a safe and secure DNS library. This is the foundational DNS protocol library for all Trust-DNS projects.")
    (description
     "This package provides Trust-DNS is a safe and secure DNS library.  This is the foundational DNS
protocol library for all Trust-DNS projects.")
    (license (list license:expat license:asl2.0))))

(define-public rust-trust-dns-https-0.20
  (package
    (name "rust-trust-dns-https")
    (version "0.20.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "trust-dns-https" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0l6x06vpm0fgcrldvk23ma0rd2xvd70f55ffncy0cqjqxnvwgbg2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytes" ,rust-bytes-1)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-data-encoding" ,rust-data-encoding-2)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-h2" ,rust-h2-0.3)
                       ("rust-http" ,rust-http-0.2)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-rustls" ,rust-rustls-0.19)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-rustls" ,rust-tokio-rustls-0.22)
                       ("rust-trust-dns-proto" ,rust-trust-dns-proto-0.20)
                       ("rust-trust-dns-rustls" ,rust-trust-dns-rustls-0.20)
                       ("rust-webpki" ,rust-webpki-0.21)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.21))))
    (home-page "http://www.trust-dns.org/index.html")
    (synopsis
     "Trust-DNS is a safe and secure DNS library. This is an extension for the Trust-DNS client to use DNS over HTTPS.")
    (description
     "This package provides Trust-DNS is a safe and secure DNS library.  This is an extension for the
Trust-DNS client to use DNS over HTTPS.")
    (license (list license:expat license:asl2.0))))

(define-public rust-trust-dns-client-0.20
  (package
    (name "rust-trust-dns-client")
    (version "0.20.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "trust-dns-client" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1p093w3g3gmqnzpm79750mghhl1p2hwh7c5bliw9nmg0pnwzjkjv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-data-encoding" ,rust-data-encoding-2)
                       ("rust-futures-channel" ,rust-futures-channel-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-openssl" ,rust-openssl-0.10)
                       ("rust-radix-trie" ,rust-radix-trie-0.2)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-ring" ,rust-ring-0.16)
                       ("rust-rustls" ,rust-rustls-0.19)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-trust-dns-https" ,rust-trust-dns-https-0.20)
                       ("rust-trust-dns-proto" ,rust-trust-dns-proto-0.20)
                       ("rust-webpki" ,rust-webpki-0.21))))
    (home-page "https://trust-dns.org/")
    (synopsis
     "Trust-DNS is a safe and secure DNS library. This is the Client library with DNSSEC support.
 DNSSEC with NSEC validation for negative records, is complete. The client supports
 dynamic DNS with SIG0 authenticated requests, implementing easy to use high level
 funtions. Trust-DNS is based on the Tokio and Futures libraries, which means
 it should be easily integrated into other software that also use those
 libraries.")
    (description
     "This package provides Trust-DNS is a safe and secure DNS library.  This is the Client library with
DNSSEC support.  DNSSEC with NSEC validation for negative records, is complete.
The client supports dynamic DNS with SIG0 authenticated requests, implementing
easy to use high level funtions.  Trust-DNS is based on the Tokio and Futures
libraries, which means it should be easily integrated into other software that
also use those libraries.")
    (license (list license:expat license:asl2.0))))

(define-public rust-hyper-system-resolver-0.5
  (package
    (name "rust-hyper-system-resolver")
    (version "0.5.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "hyper-system-resolver" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1rz26r07fgjmlcppnvwj82kl5w0hyc500rcz45r9vaxns32jdskf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-derive-builder" ,rust-derive-builder-0.9)
                       ("rust-dns-lookup" ,rust-dns-lookup-1)
                       ("rust-hyper" ,rust-hyper-0.14)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tower-service" ,rust-tower-service-0.3)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-winapi" ,rust-winapi-0.3))))
    (home-page "https://github.com/MOZGIII/hyper-system-resolver")
    (synopsis "An advanced system resolver for use with hyper")
    (description
     "This package provides An advanced system resolver for use with hyper.")
    (license license:expat)))

(define-public rust-dns-lookup-1
  (package
    (name "rust-dns-lookup")
    (version "1.0.8")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "dns-lookup" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0wk877zla9gdns5f1zgrxwzpi0abj2ld2n54a6dqsln4ab4szv2k"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-clippy" ,rust-clippy-0.0.302)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-socket2" ,rust-socket2-0.4)
                       ("rust-winapi" ,rust-winapi-0.3))))
    (home-page "https://github.com/keeperofdakeys/dns-lookup/")
    (synopsis
     "simple dns resolving api, much like rust's unstable api. Also includes getaddrinfo and getnameinfo wrappers for libc variants.")
    (description
     "This package provides a simple dns resolving api, much like rust's unstable api.
 Also includes getaddrinfo and getnameinfo wrappers for libc variants.")
    (license (list license:expat license:asl2.0))))

(define-public rust-public-ip-0.2
  (package
    (name "rust-public-ip")
    (version "0.2.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "public-ip" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16j15xpbfn0fpjixpkgfg1m9afhvpkw9k8k3qfc35nb2abdl0k3v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-dns-lookup" ,rust-dns-lookup-1)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-http" ,rust-http-0.2)
                       ("rust-hyper" ,rust-hyper-0.14)
                       ("rust-hyper-system-resolver" ,rust-hyper-system-resolver-0.5)
                       ("rust-pin-project-lite" ,rust-pin-project-lite-0.2)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-tracing-futures" ,rust-tracing-futures-0.2)
                       ("rust-trust-dns-client" ,rust-trust-dns-client-0.20)
                       ("rust-trust-dns-proto" ,rust-trust-dns-proto-0.20))
       #:cargo-development-inputs (("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/avitex/rust-public-ip")
    (synopsis "Find the public IP address of a device")
    (description
     "This package provides Find the public IP address of a device.")
    (license license:expat)))

(define-public rust-twilight-validate-0.15
  (package
    (name "rust-twilight-validate")
    (version "0.15.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "twilight-validate" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "192xh7ds25dk4rss5vbdn5x13a6kafdqj0ba16bkm3axb6nxb59m"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-twilight-model" ,rust-twilight-model-0.15))))
    (home-page "https://twilight.rs/")
    (synopsis "Functions and constants for validating request parameters")
    (description
     "This package provides functions and constants for validating request parameters.")
    (license license:isc)))

(define-public rust-twilight-model-0.15
  (package
    (name "rust-twilight-model")
    (version "0.15.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "twilight-model" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "06jaif3y90xrglm8p8h01zyzsqbvbngwiyjs74hv9cqp907xasr7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-value" ,rust-serde-value-0.7)
                       ("rust-serde-repr" ,rust-serde-repr-0.1)
                       ("rust-time" ,rust-time-0.3))))
    (home-page "https://twilight.rs/chapter_1_crates/section_1_model.html")
    (synopsis "Discord API models for the Twilight ecosystem")
    (description
     "This package provides Discord API models for the Twilight ecosystem.")
    (license license:isc)))

(define-public rust-twilight-http-ratelimiting-0.15
  (package
    (name "rust-twilight-http-ratelimiting")
    (version "0.15.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "twilight-http-ratelimiting" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0azh4z0kq3c499a4749yrkkaqrp2nyqf8b3jbddqs2pn7ws06jma"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-http" ,rust-http-0.2)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tracing" ,rust-tracing-0.1))))
    (home-page "https://twilight.rs/")
    (synopsis
     "Discord REST API ratelimiter implementations for the Twilight ecosystem")
    (description
     "This package provides Discord REST API ratelimiter implementations for the Twilight ecosystem.")
    (license license:isc)))

(define-public rust-hyper-trust-dns-0.5
  (package
    (name "rust-hyper-trust-dns")
    (version "0.5.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "hyper-trust-dns" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ga0v2yjk2rpz3np2yhbsz3f9nns5fl9dxhi833w02albj5z1shd"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-hyper" ,rust-hyper-0.14)
                       ("rust-hyper-rustls" ,rust-hyper-rustls-0.23)
                       ("rust-hyper-tls" ,rust-hyper-tls-0.5)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-trust-dns-resolver" ,rust-trust-dns-resolver-0.22))))
    (home-page "https://github.com/Gelbpunkt/hyper-trust-dns")
    (synopsis
     "HTTP/HTTPS connectors for hyper that use trust-dns' DNS resolver")
    (description
     "This package provides HTTP/HTTPS connectors for hyper that use trust-dns DNS resolver.")
    (license license:expat)))

(define-public rust-twilight-http-0.15
  (package
    (name "rust-twilight-http")
    (version "0.15.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "twilight-http" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1n1dzz9a16rm9k6b2f67dvmq6zkc96wwzhsdzkdi92552bcmb2y7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-brotli" ,rust-brotli-3)
                       ("rust-hyper" ,rust-hyper-0.14)
                       ("rust-hyper-rustls" ,rust-hyper-rustls-0.23)
                       ("rust-hyper-tls" ,rust-hyper-tls-0.5)
                       ("rust-hyper-trust-dns" ,rust-hyper-trust-dns-0.5)
                       ("rust-percent-encoding" ,rust-percent-encoding-2)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-simd-json" ,rust-simd-json-0.10)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-twilight-http-ratelimiting" ,rust-twilight-http-ratelimiting-0.15)
                       ("rust-twilight-model" ,rust-twilight-model-0.15)
                       ("rust-twilight-validate" ,rust-twilight-validate-0.15))))
    (home-page "https://twilight.rs/chapter_1_crates/section_2_http.html")
    (synopsis "Discord REST API client for the Twilight ecosystem")
    (description
     "This package provides Discord REST API client for the Twilight ecosystem.")
    (license license:isc)))

(define-public rust-twilight-gateway-queue-0.15
  (package
    (name "rust-twilight-gateway-queue")
    (version "0.15.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "twilight-gateway-queue" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0nmk7idqbf854h4j0si0r0hl0ly9a427bv9y732rpl71m1yp8wrh"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-tokio" ,rust-tokio-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-twilight-http" ,rust-twilight-http-0.15))))
    (home-page "https://twilight.rs/")
    (synopsis
     "Discord Gateway connection queue implementation for the Twilight ecosystem")
    (description
     "This package provides Discord Gateway connection queue implementation for the Twilight ecosystem.")
    (license license:isc)))

(define-public rust-tungstenite-0.18
  (package
    (name "rust-tungstenite")
    (version "0.18.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tungstenite" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1207jv8ciklgnqwjhxc1c1xhplrfab231191apyz0k6d56vnmvih"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-base64" ,rust-base64-0.13)
                       ("rust-byteorder" ,rust-byteorder-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-http" ,rust-http-0.2)
                       ("rust-httparse" ,rust-httparse-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-rustls" ,rust-rustls-0.20)
                       ("rust-rustls-native-certs" ,rust-rustls-native-certs-0.6)
                       ("rust-sha1" ,rust-sha1-0.10)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-url" ,rust-url-2)
                       ("rust-utf-8" ,rust-utf-8-0.7)
                       ("rust-webpki" ,rust-webpki-0.22)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.22))))
    (home-page "https://github.com/snapview/tungstenite-rs")
    (synopsis "Lightweight stream-based WebSocket implementation")
    (description
     "This package provides Lightweight stream-based @code{WebSocket} implementation.")
    (license (list license:expat license:asl2.0))))

(define-public rust-tokio-tungstenite-0.18
  (package
    (name "rust-tokio-tungstenite")
    (version "0.18.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tokio-tungstenite" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1z8bxhq6d1ndh4x914wwk72l93ha1sl0jmnb6knvqiqi869rqcal"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-rustls" ,rust-rustls-0.20)
                       ("rust-rustls-native-certs" ,rust-rustls-native-certs-0.6)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-native-tls" ,rust-tokio-native-tls-0.3)
                       ("rust-tokio-rustls" ,rust-tokio-rustls-0.23)
                       ("rust-tungstenite" ,rust-tungstenite-0.18)
                       ("rust-webpki" ,rust-webpki-0.22)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.22))))
    (home-page "https://github.com/snapview/tokio-tungstenite")
    (synopsis
     "Tokio binding for Tungstenite, the Lightweight stream-based WebSocket implementation")
    (description
     "This package provides Tokio binding for Tungstenite, the Lightweight stream-based @code{WebSocket}
implementation.")
    (license license:expat)))

(define-public rust-value-trait-0.6
  (package
    (name "rust-value-trait")
    (version "0.6.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "value-trait" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0pllqs0gq31cwn4xh4zd1m4ls4q5cx58vi6ad6wn64mhrv4bd989"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-abi-stable" ,rust-abi-stable-0.11)
                       ("rust-float-cmp" ,rust-float-cmp-0.9)
                       ("rust-halfbrown" ,rust-halfbrown-0.2)
                       ("rust-hashbrown" ,rust-hashbrown-0.13)
                       ("rust-itoa" ,rust-itoa-1)
                       ("rust-ryu" ,rust-ryu-1))))
    (home-page "https://github.com/simd-lite/value-trait")
    (synopsis "Traits to deal with JSONesque values")
    (description "This package provides Traits to deal with JSONesque values.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-simd-json-0.10
  (package
    (name "rust-simd-json")
    (version "0.10.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "simd-json" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1syqpnymzlafk289zcvl43gf4asp0bsxdpf4gy36a2a05ky1vsl0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-alloc-counter" ,rust-alloc-counter-0.0.4)
                       ("rust-beef" ,rust-beef-0.5)
                       ("rust-colored" ,rust-colored-2)
                       ("rust-getopts" ,rust-getopts-0.2)
                       ("rust-getrandom" ,rust-getrandom-0.2)
                       ("rust-halfbrown" ,rust-halfbrown-0.2)
                       ("rust-jemallocator" ,rust-jemallocator-0.5)
                       ("rust-lexical-core" ,rust-lexical-core-0.8)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-perfcnt" ,rust-perfcnt-0.8)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-simdutf8" ,rust-simdutf8-0.1)
                       ("rust-value-trait" ,rust-value-trait-0.6))))
    (home-page "https://github.com/simd-lite/simd-json")
    (synopsis "High performance JSON parser based on a port of simdjson")
    (description
     "This package provides High performance JSON parser based on a port of simdjson.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-twilight-gateway-0.15
  (package
    (name "rust-twilight-gateway")
    (version "0.15.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "twilight-gateway" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "11kxskm88hqzrvp298qqldwl4h131h3lqd4k1ygabd0k5dz5rgih"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-1)
                       ("rust-flate2" ,rust-flate2-1)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-rustls" ,rust-rustls-0.20)
                       ("rust-rustls-native-certs" ,rust-rustls-native-certs-0.6)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-simd-json" ,rust-simd-json-0.10)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-tungstenite" ,rust-tokio-tungstenite-0.18)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-twilight-gateway-queue" ,rust-twilight-gateway-queue-0.15)
                       ("rust-twilight-http" ,rust-twilight-http-0.15)
                       ("rust-twilight-model" ,rust-twilight-model-0.15)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.22))))
    (home-page "https://twilight.rs/chapter_1_crates/section_3_gateway.html")
    (synopsis "Discord Gateway implementation for the Twilight ecosystem")
    (description
     "This package provides Discord Gateway implementation for the Twilight ecosystem.")
    (license license:isc)))

(define-public rust-extended-0.1
  (package
    (name "rust-extended")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "extended" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0r830ak1a9775i9yl5lljm29zbnlncw7xlfz35mhgjrz43c775mg"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/depp/extended-rs")
    (synopsis "Extended precision 80-bit floating-point numbers (f80).")
    (description
     "This package provides Extended precision 80-bit floating-point numbers (f80).")
    (license license:expat)))

(define-public rust-symphonia-format-riff-0.5
  (package
    (name "rust-symphonia-format-riff")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-format-riff" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0l2zs6zl7q15jhsk9j1lahs2j29k5kkcn5bi9dzr6bwn5wivxxq5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-extended" ,rust-extended-0.1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-metadata" ,rust-symphonia-metadata-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust RIFF demuxer (a part of project Symphonia)")
    (description
     "This package provides Pure Rust RIFF demuxer (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-format-ogg-0.5
  (package
    (name "rust-symphonia-format-ogg")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-format-ogg" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0cd9py2xgx211qvwl9sw8n5l5vgd55vwcmqizh0cyssii5bm18xd"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-metadata" ,rust-symphonia-metadata-0.5)
                       ("rust-symphonia-utils-xiph" ,rust-symphonia-utils-xiph-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust OGG demuxer (a part of project Symphonia)")
    (description
     "This package provides Pure Rust OGG demuxer (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-format-mkv-0.5
  (package
    (name "rust-symphonia-format-mkv")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-format-mkv" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0vrxzr95d1xk2l5jarp7k2935s5ybsyrawwkr4nqixq0l5qk9d0v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-metadata" ,rust-symphonia-metadata-0.5)
                       ("rust-symphonia-utils-xiph" ,rust-symphonia-utils-xiph-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust MKV/WebM demuxer (a part of project Symphonia)")
    (description
     "This package provides Pure Rust MKV/@code{WebM} demuxer (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-format-isomp4-0.5
  (package
    (name "rust-symphonia-format-isomp4")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-format-isomp4" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0i68dnhp3q7hv4i51hryw0c75i4l3fx85ffrwphhrrcpsrwg3zdb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-encoding-rs" ,rust-encoding-rs-0.8)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-metadata" ,rust-symphonia-metadata-0.5)
                       ("rust-symphonia-utils-xiph" ,rust-symphonia-utils-xiph-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust ISO/MP4 demuxer (a part of project Symphonia)")
    (description
     "This package provides Pure Rust ISO/MP4 demuxer (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-format-caf-0.5
  (package
    (name "rust-symphonia-format-caf")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-format-caf" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0l5gjy8asdcw8p2k9xqw0hc8npcz0wrv2wgy55d2k253jv39jg74"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-metadata" ,rust-symphonia-metadata-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust CAF demuxer (a part of project Symphonia)")
    (description
     "This package provides Pure Rust CAF demuxer (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-codec-vorbis-0.5
  (package
    (name "rust-symphonia-codec-vorbis")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-codec-vorbis" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0c4z98b8yg2kws3pknw7ipvvca911j3y5xq7n0r6f2kanigpd62s"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-utils-xiph" ,rust-symphonia-utils-xiph-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust Vorbis decoder (a part of project Symphonia)")
    (description
     "This package provides Pure Rust Vorbis decoder (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-codec-pcm-0.5
  (package
    (name "rust-symphonia-codec-pcm")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-codec-pcm" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16zq2s8zf0rs6070y3sfyscvm9z1riqvxcbv9plcbsy2axqad5gk"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust PCM audio decoder (a part of project Symphonia)")
    (description
     "This package provides Pure Rust PCM audio decoder (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-codec-alac-0.5
  (package
    (name "rust-symphonia-codec-alac")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-codec-alac" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1wrq1s6w029bz7lqj08q87i375wvzl78nsj70qll224scik6d2id"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust ALAC decoder (a part of project Symphonia)")
    (description
     "This package provides Pure Rust ALAC decoder (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-codec-adpcm-0.5
  (package
    (name "rust-symphonia-codec-adpcm")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-codec-adpcm" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03va885srhrzfz31jvxh2rgr9crnmmlvxmbkx4bdcz1jqgm1ykn9"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust ADPCM audio decoder (a part of project Symphonia)")
    (description
     "This package provides Pure Rust ADPCM audio decoder (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-codec-aac-0.5
  (package
    (name "rust-symphonia-codec-aac")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-codec-aac" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0w1ga9c7m5bb11rc9bpnjb5g9bqms4x69slix3ikw3dd8nsjbgyd"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust AAC decoder (a part of project Symphonia)")
    (description
     "This package provides Pure Rust AAC decoder (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-bundle-mp3-0.5
  (package
    (name "rust-symphonia-bundle-mp3")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-bundle-mp3" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1m062zkxq2cbwqxbm3qp4qvgpc9hm49g23vgdc4zpwghf2p2l760"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-metadata" ,rust-symphonia-metadata-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis
     "Pure Rust MP1, MP2, and MP3 demuxer and decoder (a part of project Symphonia)")
    (description
     "This package provides Pure Rust MP1, MP2, and MP3 demuxer and decoder (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-utils-xiph-0.5
  (package
    (name "rust-symphonia-utils-xiph")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-utils-xiph" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1zhhs1p0h6wdcgcwfqpmqq07n8v2wvn50razvapr36d41xc74i28"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-metadata" ,rust-symphonia-metadata-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Project Symphonia utilities for Xiph codecs and formats")
    (description
     "This package provides Project Symphonia utilities for Xiph codecs and formats.")
    (license license:mpl2.0)))

(define-public rust-symphonia-metadata-0.5
  (package
    (name "rust-symphonia-metadata")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-metadata" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0g02lhhyf6yyxm7bynx5b9fn2ha39y8fp6cfn72qj05186c2nqmw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-encoding-rs" ,rust-encoding-rs-0.8)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Project Symphonia multimedia tag and metadata readers")
    (description
     "This package provides Project Symphonia multimedia tag and metadata readers.")
    (license license:mpl2.0)))

(define-public rust-symphonia-core-0.5
  (package
    (name "rust-symphonia-core")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-core" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1hr2w2a217vq4lpghszmsdwxr5ilh5d1ysfm3cixbirxkrvhd0vr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-bitflags" ,rust-bitflags-1)
                       ("rust-bytemuck" ,rust-bytemuck-1)
                       ("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-rustfft" ,rust-rustfft-6))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Project Symphonia shared structs, traits, and features")
    (description
     "This package provides Project Symphonia shared structs, traits, and features.")
    (license license:mpl2.0)))

(define-public rust-symphonia-bundle-flac-0.5
  (package
    (name "rust-symphonia-bundle-flac")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia-bundle-flac" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "15xxncx6gfh7jwvxvqqw4f8x9ic4bfzpyv3s77a0hwwa54s4zqvj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-log" ,rust-log-0.4)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-metadata" ,rust-symphonia-metadata-0.5)
                       ("rust-symphonia-utils-xiph" ,rust-symphonia-utils-xiph-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis
     "Pure Rust FLAC demuxer and decoder (a part of project Symphonia)")
    (description
     "This package provides Pure Rust FLAC demuxer and decoder (a part of project Symphonia).")
    (license license:mpl2.0)))

(define-public rust-symphonia-0.5
  (package
    (name "rust-symphonia")
    (version "0.5.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "symphonia" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1agmsnmzgsmvd70hq760nvkjrb52nnjmz5hgn1xp6x7fwwm98p41"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-lazy-static" ,rust-lazy-static-1)
                       ("rust-symphonia-bundle-flac" ,rust-symphonia-bundle-flac-0.5)
                       ("rust-symphonia-bundle-mp3" ,rust-symphonia-bundle-mp3-0.5)
                       ("rust-symphonia-codec-aac" ,rust-symphonia-codec-aac-0.5)
                       ("rust-symphonia-codec-adpcm" ,rust-symphonia-codec-adpcm-0.5)
                       ("rust-symphonia-codec-alac" ,rust-symphonia-codec-alac-0.5)
                       ("rust-symphonia-codec-pcm" ,rust-symphonia-codec-pcm-0.5)
                       ("rust-symphonia-codec-vorbis" ,rust-symphonia-codec-vorbis-0.5)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-symphonia-format-caf" ,rust-symphonia-format-caf-0.5)
                       ("rust-symphonia-format-isomp4" ,rust-symphonia-format-isomp4-0.5)
                       ("rust-symphonia-format-mkv" ,rust-symphonia-format-mkv-0.5)
                       ("rust-symphonia-format-ogg" ,rust-symphonia-format-ogg-0.5)
                       ("rust-symphonia-format-riff" ,rust-symphonia-format-riff-0.5)
                       ("rust-symphonia-metadata" ,rust-symphonia-metadata-0.5))))
    (home-page "https://github.com/pdeljanov/Symphonia")
    (synopsis "Pure Rust media container and audio decoding library")
    (description
     "This package provides Pure Rust media container and audio decoding library.")
    (license license:mpl2.0)))

(define-public rust-async-compat-0.2
  (package
    (name "rust-async-compat")
    (version "0.2.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "async-compat" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1h4gqla7s8wczivqskbxg7nl3qj0svd6yf9fjssgg8wnwfyr9avv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-io" ,rust-futures-io-0.3)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-pin-project-lite" ,rust-pin-project-lite-0.2)
                       ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/smol-rs/async-compat")
    (synopsis "Compatibility adapter between tokio and futures")
    (description
     "This package provides Compatibility adapter between tokio and futures.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-streamcatcher-1
  (package
    (name "rust-streamcatcher")
    (version "1.0.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "streamcatcher" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0agazjs1sja5n41jqc7rfcmcm4935pxihqnsixsv1as9qdalfrki"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-async-compat" ,rust-async-compat-0.2)
                       ("rust-async-std" ,rust-async-std-1)
                       ("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-crossbeam-utils" ,rust-crossbeam-utils-0.8)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-loom" ,rust-loom-0.5)
                       ("rust-smol" ,rust-smol-1)
                       ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/FelixMcFelix/streamcatcher")
    (synopsis
     "thread-safe, shared (asynchronous), almost-lockless stream buffer.")
    (description
     "This package provides a thread-safe, shared (asynchronous), almost-lockless
stream buffer.")
    (license (list license:expat license:asl2.0))))

(define-public rust-tokio-macros-2
  (package
    (name "rust-tokio-macros")
    (version "2.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tokio-macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lnpg14h1v3fh2jvnc8cz7cjf0m7z1xgkwfpcyy632g829imjgb9"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://tokio.rs")
    (synopsis "Tokio's proc macros.")
    (description "This package provides Tokio's proc macros.")
    (license license:expat)))

(define-public rust-hermit-abi-0.3
  (package
    (name "rust-hermit-abi")
    (version "0.3.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "hermit-abi" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "092hxjbjnq5fmz66grd9plxd0sh6ssg5fhgwwwqbrzgzkjwdycfj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-compiler-builtins" ,rust-compiler-builtins-0.1)
                       ("rust-rustc-std-workspace-alloc" ,rust-rustc-std-workspace-alloc-1)
                       ("rust-rustc-std-workspace-core" ,rust-rustc-std-workspace-core-1))))
    (home-page "https://github.com/hermit-os/hermit-rs")
    (synopsis "Hermit system calls definitions")
    (description "This package provides Hermit system calls definitions.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mio-1
  (package
    (name "rust-mio")
    (version "1.0.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mio" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1r5g65s5acsx440m0a3pylclbrd0dqz93hg15k9crpllsdbf8sa5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-hermit-abi" ,rust-hermit-abi-0.3)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-wasi" ,rust-wasi-0.11)
                       ("rust-windows-sys" ,rust-windows-sys-0.52))))
    (home-page "https://github.com/tokio-rs/mio")
    (synopsis "Lightweight non-blocking I/O")
    (description "This package provides Lightweight non-blocking I/O.")
    (license license:expat)))

(define-public rust-patricia-tree-0.8
  (package
    (name "rust-patricia-tree")
    (version "0.8.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "patricia_tree" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0s5fya6rvgg2gxxp5mbv0xdq8jqikps1sc6snk23zrgzkd9z9wii"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-2)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/sile/patricia_tree")
    (synopsis "Memory-efficient data structures based on patricia tree")
    (description
     "This package provides Memory-efficient data structures based on patricia tree.")
    (license license:expat)))

(define-public rust-strum-macros-0.17
  (package
    (name "rust-strum-macros")
    (version "0.17.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "strum_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "043ikzkjnf1acxpqpms6cl5v3qzs7ji622lyymjw8rq3a8x1cvjy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-heck" ,rust-heck-0.3)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/Peternator7/strum")
    (synopsis "Helpful macros for working with enums and strings")
    (description
     "This package provides Helpful macros for working with enums and strings.")
    (license license:expat)))

(define-public rust-strum-0.17
  (package
    (name "rust-strum")
    (version "0.17.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "strum" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0b3dn24vhbqqnd1pjighlivfn3fjgrg6r4a7wgs15dsk1n1gn3jk"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-strum-macros" ,rust-strum-macros-0.17))))
    (home-page "https://github.com/Peternator7/strum")
    (synopsis "Helpful macros for working with enums and strings")
    (description
     "This package provides Helpful macros for working with enums and strings.")
    (license license:expat)))

(define-public rust-no-std-compat-0.2
  (package
    (name "rust-no-std-compat")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "no-std-compat" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1d3n0a9hjjjp4dw974icn5flswcjnzn91n20j92n4kghlw4h49yz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-hashbrown" ,rust-hashbrown-0.6))))
    (home-page "https://gitlab.com/jD91mZM2/no-std-compat")
    (synopsis
     "`#![no_std]` compatibility layer that will make porting your crate to no_std *easy*.")
    (description
     "This package provides a `#![no_std]` compatibility layer that will make porting
your crate to no_std *easy*.")
    (license license:expat)))

(define-public rust-stable-vec-0.4
  (package
    (name "rust-stable-vec")
    (version "0.4.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "stable-vec" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1j4yvhqkb16h3aab4q471mv8in7bgh11k147xhxji1z05hmg7pyi"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-no-std-compat" ,rust-no-std-compat-0.2))))
    (home-page "https://github.com/LukasKalbertodt/stable-vec")
    (synopsis
     "Vec-like collection which guarantees stable indices and features O(1)
element deletion (semantically similar to `Vec<Option<T>>`). Useful for
allocations in graphs or similar data structures.")
    (description
     "This package provides a Vec-like collection which guarantees stable indices and
features O(1) element deletion (semantically similar to `Vec<Option<T>>`).
Useful for allocations in graphs or similar data structures.")
    (license (list license:expat license:asl2.0))))

(define-public rust-hashmap-derive-0.1
  (package
    (name "rust-hashmap-derive")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "hashmap_derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "10r8y6bajdva17kb1hzpkxwff56a4da0kb2x4ssk3k3j7qbvyc7v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/Luro02/shorthand")
    (synopsis "proc_macro to derive the FromMap trait")
    (description
     "This package provides a proc_macro to derive the @code{FromMap} trait.")
    (license (list license:unlicense license:expat license:asl2.0))))

(define-public rust-from-map-0.1
  (package
    (name "rust-from-map")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "from_map" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "000p626mhzgffv91vphm703q6xp04igq02s2xs6gyi84mci13wwr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-hashmap-derive" ,rust-hashmap-derive-0.1))))
    (home-page "https://github.com/Luro02/shorthand")
    (synopsis "crate that exposes the `FromMap` trait")
    (description
     "This package provides a crate that exposes the `@code{FromMap`} trait.")
    (license (list license:unlicense license:expat license:asl2.0))))

(define-public rust-shorthand-0.1
  (package
    (name "rust-shorthand")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "shorthand" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0cc9y5y2jb7k86qdxm61q0288xiy2ypjwcvhy482c8fqhpwpfks7"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-from-map" ,rust-from-map-0.1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/Luro02/shorthand")
    (synopsis "proc_macro to derive getter and setter for structs")
    (description
     "This package provides a proc_macro to derive getter and setter for structs.")
    (license (list license:expat license:asl2.0))))

(define-public rust-hls-m3u8-0.4
  (package
    (name "rust-hls-m3u8")
    (version "0.4.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "hls_m3u8" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1flgg0ckrdzgn45106vm21mjzkz598xca6nf8qak1dcjgzgd4aqf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-backtrace" ,rust-backtrace-0.3)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-derive-builder" ,rust-derive-builder-0.9)
                       ("rust-derive-more" ,rust-derive-more-0.99)
                       ("rust-hex" ,rust-hex-0.4)
                       ("rust-shorthand" ,rust-shorthand-0.1)
                       ("rust-stable-vec" ,rust-stable-vec-0.4)
                       ("rust-strum" ,rust-strum-0.17)
                       ("rust-thiserror" ,rust-thiserror-1))))
    (home-page "https://github.com/sile/hls_m3u8")
    (synopsis "HLS m3u8 parser/generator")
    (description "This package provides HLS m3u8 parser/generator.")
    (license (list license:expat license:asl2.0))))

(define-public rust-stream-lib-0.4
  (package
    (name "rust-stream-lib")
    (version "0.4.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "stream_lib" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "19x5k14agdkr9hycd4bmhix1h24iwbj11lv1pxxf2m3hbbmi0gzs"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytes" ,rust-bytes-1)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-hls-m3u8" ,rust-hls-m3u8-0.4)
                       ("rust-patricia-tree" ,rust-patricia-tree-0.8)
                       ("rust-reqwest" ,rust-reqwest-0.11)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-url" ,rust-url-2))))
    (home-page "https://github.com/Erk-/rsget")
    (synopsis "Tool to download differnt types of streams")
    (description
     "This package provides Tool to download differnt types of streams.")
    (license license:isc)))

(define-public rust-uwl-0.6
  (package
    (name "rust-uwl")
    (version "0.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "uwl" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1l0spdpn879wpf440x4cdsbz5dilp5ihfsxsqkn2dmkhrbh07gzl"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/acdenisSK/uwl")
    (synopsis "management stream for bytes and characters")
    (description
     "This package provides a management stream for bytes and characters.")
    (license (list license:expat license:asl2.0))))

(define-public rust-typesize-derive-0.1
  (package
    (name "rust-typesize-derive")
    (version "0.1.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typesize-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1a0ypva9lwmyfgvng2iq87j2gw2ishhm2jbysmmnh9yclk18hplh"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/GnomedDev/typesize")
    (synopsis "Internal proc-macro crate for typesize")
    (description
     "This package provides Internal proc-macro crate for typesize.")
    (license license:expat)))

(define-public rust-nonmax-0.5
  (package
    (name "rust-nonmax")
    (version "0.5.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "nonmax" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lfvyfz4falgmc9g1cbfi2wkys9wka2nfmdyga87zikf636ml2k1"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/LPGhatguy/nonmax")
    (synopsis "Numeric types that cannot hold maximum values")
    (description
     "This package provides Numeric types that cannot hold maximum values.")
    (license (list license:expat license:asl2.0))))

(define-public rust-gat-lending-iterator-0.1
  (package
    (name "rust-gat-lending-iterator")
    (version "0.1.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "gat-lending-iterator" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1agx13403m08gk13sv8qhy9m4na97bm3lgpa1m0bdmdayawpncj2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/Crazytieguy/gat-lending-iterator")
    (synopsis
     "library for iterators who's items can [mutably] reference the iterator.")
    (description
     "This package provides a library for iterators who's items can [mutably]
reference the iterator.")
    (license license:expat)))

(define-public rust-extract-map-0.1
  (package
    (name "rust-extract-map")
    (version "0.1.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "extract_map" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0s695n5yzs7wwyvaphkkscr8lfv7h94xsczg49a9qa37nnd51xxb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-gat-lending-iterator" ,rust-gat-lending-iterator-0.1)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/GnomedDev/extract_map")
    (synopsis
     "HashMap for memory efficent storage of value types which contain their own keys.")
    (description
     "This package provides a @code{HashMap} for memory efficent storage of value
types which contain their own keys.")
    (license license:expat)))

(define-public rust-typesize-0.1
  (package
    (name "rust-typesize")
    (version "0.1.7")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typesize" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03zf3k54hbn0mvcx1axv5b5crvlb435h9rwr7vv7dg09qx14hw7b"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-extract-map" ,rust-extract-map-0.1)
                       ("rust-halfbrown" ,rust-halfbrown-0.2)
                       ("rust-hashbrown" ,rust-hashbrown-0.14)
                       ("rust-mini-moka" ,rust-mini-moka-0.10)
                       ("rust-nonmax" ,rust-nonmax-0.5)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-secrecy" ,rust-secrecy-0.8)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-simd-json" ,rust-simd-json-0.13)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-typesize-derive" ,rust-typesize-derive-0.1)
                       ("rust-url" ,rust-url-2))))
    (home-page "https://github.com/GnomedDev/typesize")
    (synopsis
     "library to fetch an accurate estimate of the total memory usage of a value.")
    (description
     "This package provides a library to fetch an accurate estimate of the total
memory usage of a value.")
    (license license:expat)))

(define-public rust-typemap-rev-0.3
  (package
    (name "rust-typemap-rev")
    (version "0.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typemap_rev" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "161935l8j5jxzjz64g4z21z3x7aj9ljhadjwdbqilf2p2868pc3l"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/bdashore3/typemap_rev")
    (synopsis "hashmap, but stores types as keys")
    (description "This package provides a hashmap, but stores types as keys.")
    (license license:isc)))

(define-public rust-tungstenite-0.21
  (package
    (name "rust-tungstenite")
    (version "0.21.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tungstenite" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1qaphb5kgwgid19p64grhv2b9kxy7f1059yy92l9kwrlx90sdwcy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-byteorder" ,rust-byteorder-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-data-encoding" ,rust-data-encoding-2)
                       ("rust-http" ,rust-http-1)
                       ("rust-httparse" ,rust-httparse-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-rustls" ,rust-rustls-0.22)
                       ("rust-rustls-native-certs" ,rust-rustls-native-certs-0.7)
                       ("rust-rustls-pki-types" ,rust-rustls-pki-types-1)
                       ("rust-sha1" ,rust-sha1-0.10)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-url" ,rust-url-2)
                       ("rust-utf-8" ,rust-utf-8-0.7)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.26))))
    (home-page "https://github.com/snapview/tungstenite-rs")
    (synopsis "Lightweight stream-based WebSocket implementation")
    (description
     "This package provides Lightweight stream-based @code{WebSocket} implementation.")
    (license (list license:expat license:asl2.0))))

(define-public rust-tokio-tungstenite-0.21
  (package
    (name "rust-tokio-tungstenite")
    (version "0.21.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tokio-tungstenite" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0f5wj0crsx74rlll97lhw0wk6y12nhdnqvmnjx002hjn08fmcfy8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-rustls" ,rust-rustls-0.22)
                       ("rust-rustls-native-certs" ,rust-rustls-native-certs-0.7)
                       ("rust-rustls-pki-types" ,rust-rustls-pki-types-1)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-native-tls" ,rust-tokio-native-tls-0.3)
                       ("rust-tokio-rustls" ,rust-tokio-rustls-0.25)
                       ("rust-tungstenite" ,rust-tungstenite-0.21)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.26))))
    (home-page "https://github.com/snapview/tokio-tungstenite")
    (synopsis
     "Tokio binding for Tungstenite, the Lightweight stream-based WebSocket implementation")
    (description
     "This package provides Tokio binding for Tungstenite, the Lightweight stream-based @code{WebSocket}
implementation.")
    (license license:expat)))

(define-public rust-typewit-proc-macros-1
  (package
    (name "rust-typewit-proc-macros")
    (version "1.8.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typewit_proc_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1mlkh4mhbn4b7xg9640blk74bm5ddaa44ihvl0sljw1w5gm86sp3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/rodrimati1992/typewit/")
    (synopsis "implementation detail of typewit")
    (description "This package provides implementation detail of typewit.")
    (license license:zlib)))

(define-public rust-typewit-1
  (package
    (name "rust-typewit")
    (version "1.9.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "typewit" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "093fgb1q5n48vr4nj3hggbhfi6jzab5048scs6jz1ynalgk9myy6"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-typewit-proc-macros" ,rust-typewit-proc-macros-1))))
    (home-page "https://github.com/rodrimati1992/typewit/")
    (synopsis
     "type-witness-based abstractions, mostly for emulating polymorphism in const fns")
    (description
     "This package provides type-witness-based abstractions, mostly for emulating polymorphism in const fns.")
    (license license:zlib)))

(define-public rust-const-panic-proc-macros-0.2
  (package
    (name "rust-const-panic-proc-macros")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "const_panic_proc_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1addx3a8vi02cdak3ygrqivv02jj73251h85x49aic78yznrhlrr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1)
                       ("rust-unicode-xid" ,rust-unicode-xid-0.2))))
    (home-page "https://github.com/rodrimati1992/const_panic/")
    (synopsis "Implementation detail of the `const_panic` crate")
    (description
     "This package provides Implementation detail of the `const_panic` crate.")
    (license license:zlib)))

(define-public rust-const-panic-0.2
  (package
    (name "rust-const-panic")
    (version "0.2.8")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "const_panic" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16w72mnzjqgwfhlq8cqm6xhd2n6lc1wan08987izv1pcxhwz4lb0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-const-panic-proc-macros" ,rust-const-panic-proc-macros-0.2)
                       ("rust-typewit" ,rust-typewit-1))))
    (home-page "https://github.com/rodrimati1992/const_panic/")
    (synopsis "const panic with formatting")
    (description "This package provides const panic with formatting.")
    (license license:zlib)))

(define-public rust-as-derive-utils-0.11
  (package
    (name "rust-as-derive-utils")
    (version "0.11.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "as_derive_utils" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1i2kwzxdhydicj9bqscz5w73nmx612yi3ha137qlr900b5j9cg7z"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-aho-corasick" ,rust-aho-corasick-0.7)
                       ("rust-bitflags" ,rust-bitflags-1)
                       ("rust-core-extensions" ,rust-core-extensions-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-ron" ,rust-ron-0.7)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-derive" ,rust-serde-derive-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/rodrimati1992/abi_stable_crates/")
    (synopsis "private derive utilities used by abi_stable and structural")
    (description
     "This package provides private derive utilities used by abi_stable and structural.")
    (license (list license:expat license:asl2.0))))

(define-public rust-abi-stable-shared-0.11
  (package
    (name "rust-abi-stable-shared")
    (version "0.11.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "abi_stable_shared" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0qrbmlypvxx3zij1c6w6yykpp5pjcfx9qr2d9lzyc8y1i1vdzddj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-core-extensions" ,rust-core-extensions-1))))
    (home-page "https://github.com/rodrimati1992/abi_stable_crates/")
    (synopsis "Implementation detail of abi_stable")
    (description "This package provides Implementation detail of abi_stable.")
    (license (list license:expat license:asl2.0))))

(define-public rust-abi-stable-derive-0.11
  (package
    (name "rust-abi-stable-derive")
    (version "0.11.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "abi_stable_derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "16780mmr2hwx8ajcq59nhvq3krv5i8r7mg41x08fx907nil885yp"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-abi-stable-shared" ,rust-abi-stable-shared-0.11)
                       ("rust-as-derive-utils" ,rust-as-derive-utils-0.11)
                       ("rust-core-extensions" ,rust-core-extensions-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-rustc-version" ,rust-rustc-version-0.4)
                       ("rust-syn" ,rust-syn-1)
                       ("rust-typed-arena" ,rust-typed-arena-2))))
    (home-page "https://github.com/rodrimati1992/abi_stable_crates/")
    (synopsis "Implementation detail of abi_stable")
    (description "This package provides Implementation detail of abi_stable.")
    (license (list license:expat license:asl2.0))))

(define-public rust-abi-stable-0.11
  (package
    (name "rust-abi-stable")
    (version "0.11.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "abi_stable" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0if428pq8ly97zi6q1842nak977rwxnj17650i8gwpxh7qnm3mk9"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-abi-stable-derive" ,rust-abi-stable-derive-0.11)
                       ("rust-abi-stable-shared" ,rust-abi-stable-shared-0.11)
                       ("rust-const-panic" ,rust-const-panic-0.2)
                       ("rust-core-extensions" ,rust-core-extensions-1)
                       ("rust-crossbeam-channel" ,rust-crossbeam-channel-0.5)
                       ("rust-generational-arena" ,rust-generational-arena-0.2)
                       ("rust-libloading" ,rust-libloading-0.7)
                       ("rust-lock-api" ,rust-lock-api-0.4)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-paste" ,rust-paste-1)
                       ("rust-repr-offset" ,rust-repr-offset-0.2)
                       ("rust-rustc-version" ,rust-rustc-version-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-derive" ,rust-serde-derive-1)
                       ("rust-serde-json" ,rust-serde-json-1))))
    (home-page "https://github.com/rodrimati1992/abi_stable_crates/")
    (synopsis
     "For doing Rust-to-Rust ffi,writing libraries loaded at program startup")
    (description
     "This package provides For doing Rust-to-Rust ffi,writing libraries loaded at program startup.")
    (license (list license:expat license:asl2.0))))

(define-public rust-value-trait-0.8
  (package
    (name "rust-value-trait")
    (version "0.8.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "value-trait" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1d3yl8g6xzp4ah4r7nrqrb1vxy9vgp53z80vy9ypjxz6q6cdpn6s"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-abi-stable" ,rust-abi-stable-0.11)
                       ("rust-float-cmp" ,rust-float-cmp-0.9)
                       ("rust-halfbrown" ,rust-halfbrown-0.2)
                       ("rust-hashbrown" ,rust-hashbrown-0.14)
                       ("rust-itoa" ,rust-itoa-1)
                       ("rust-ryu" ,rust-ryu-1))))
    (home-page "https://github.com/simd-lite/value-trait")
    (synopsis "Traits to deal with JSONesque values")
    (description "This package provides Traits to deal with JSONesque values.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-x86-0.47
  (package
    (name "rust-x86")
    (version "0.47.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "x86" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jlddyczw168mcy4a6m3nbl203rxli2vr5gcmf57s0adqf6bxdam"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bit-field" ,rust-bit-field-0.10)
                       ("rust-bitflags" ,rust-bitflags-1)
                       ("rust-csv" ,rust-csv-1)
                       ("rust-phf" ,rust-phf-0.9)
                       ("rust-phf-codegen" ,rust-phf-codegen-0.9)
                       ("rust-raw-cpuid" ,rust-raw-cpuid-10)
                       ("rust-serde-json" ,rust-serde-json-1))))
    (home-page "https://github.com/gz/rust-x86")
    (synopsis
     "Library to program x86 (amd64) hardware. Contains x86 specific data structure descriptions, data-tables, as well as convenience function to call assembly instructions typically not exposed in higher level languages")
    (description
     "This package provides Library to program x86 (amd64) hardware.  Contains x86 specific data structure
descriptions, data-tables, as well as convenience function to call assembly
instructions typically not exposed in higher level languages.")
    (license license:expat)))

(define-public rust-libc-0.1
  (package
    (name "rust-libc")
    (version "0.1.12")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "libc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "08k14zb7bw25avmaj227calcdglb4ac394kklr9nv175fp7p0ap3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/rust-lang/libc")
    (synopsis "Raw FFI bindings to platform libraries like libc.")
    (description
     "This package provides Raw FFI bindings to platform libraries like libc.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mmap-0.1
  (package
    (name "rust-mmap")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mmap" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "08xqhvr4l3rf1fkz2w4cwz3z5wd0m1jab1d34sxd4v80lr459j0b"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-libc" ,rust-libc-0.1)
                       ("rust-tempdir" ,rust-tempdir-0.3))))
    (home-page "https://github.com/rbranson/rust-mmap")
    (synopsis "library for dealing with memory-mapped I/O")
    (description
     "This package provides a library for dealing with memory-mapped I/O.")
    (license license:expat)))

(define-public rust-perfcnt-0.8
  (package
    (name "rust-perfcnt")
    (version "0.8.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "perfcnt" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "008mrdd8zjk54qg8xh8crk9is98sxv2c0kk2v25nzjkhaaazv8ab"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-mmap" ,rust-mmap-0.1)
                       ("rust-nom" ,rust-nom-4)
                       ("rust-x86" ,rust-x86-0.47))))
    (home-page "https://github.com/gz/rust-perfcnt")
    (synopsis
     "Library to configure and read hardware performance counters in rust")
    (description
     "This package provides Library to configure and read hardware performance counters in rust.")
    (license license:expat)))

(define-public rust-halfbrown-0.2
  (package
    (name "rust-halfbrown")
    (version "0.2.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "halfbrown" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0vzg46039pd730vc2hdhl09h86j4cd007awwlrf8l407hqd6d245"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-hashbrown" ,rust-hashbrown-0.14)
                       ("rust-rustc-hash" ,rust-rustc-hash-1)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/Licenser/halfbrown")
    (synopsis
     "Multi backend HashMap for higher performance on different key space sizes")
    (description
     "This package provides Multi backend @code{HashMap} for higher performance on different key space
sizes.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-alloc-counter-macro-0.0.2
  (package
    (name "rust-alloc-counter-macro")
    (version "0.0.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "alloc_counter_macro" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0nifqalryavmrdlkyv7cznp8yfjj16x0bjqzvjndw0fxk8gzhlhs"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "")
    (synopsis "The #[no_alloc] macro for the alloc_counter crate")
    (description
     "This package provides The #[no_alloc] macro for the alloc_counter crate.")
    (license (list license:expat license:asl2.0))))

(define-public rust-alloc-counter-0.0.4
  (package
    (name "rust-alloc-counter")
    (version "0.0.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "alloc_counter" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1agxzprqi37bcy9hh3clbsl3n0awbb34vrlv4rp5afib8w53m31s"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-alloc-counter-macro" ,rust-alloc-counter-macro-0.0.2)
                       ("rust-pin-utils" ,rust-pin-utils-0.1))))
    (home-page "https://gitlab.com/sio4/code/alloc-counter")
    (synopsis
     "Count allocations, reallocations, deallocations. Allow, deny, or forbid allocations on an expression or function basis")
    (description
     "This package provides Count allocations, reallocations, deallocations.  Allow, deny, or forbid
allocations on an expression or function basis.")
    (license (list license:expat license:asl2.0))))

(define-public rust-simd-json-0.13
  (package
    (name "rust-simd-json")
    (version "0.13.10")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "simd-json" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1yj3h676qd8jk80xp4vxhm0gwbbqmrik51by161s0blh7l5l632p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-alloc-counter" ,rust-alloc-counter-0.0.4)
                       ("rust-beef" ,rust-beef-0.5)
                       ("rust-colored" ,rust-colored-2)
                       ("rust-getopts" ,rust-getopts-0.2)
                       ("rust-getrandom" ,rust-getrandom-0.2)
                       ("rust-halfbrown" ,rust-halfbrown-0.2)
                       ("rust-jemallocator" ,rust-jemallocator-0.5)
                       ("rust-lexical-core" ,rust-lexical-core-0.8)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-perfcnt" ,rust-perfcnt-0.8)
                       ("rust-ref-cast" ,rust-ref-cast-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-simdutf8" ,rust-simdutf8-0.1)
                       ("rust-value-trait" ,rust-value-trait-0.8))))
    (home-page "https://github.com/simd-lite/simd-json")
    (synopsis "High performance JSON parser based on a port of simdjson")
    (description
     "This package provides High performance JSON parser based on a port of simdjson.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-serenity-voice-model-0.2
  (package
    (name "rust-serenity-voice-model")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serenity-voice-model" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0sz0h345as4ypmk2z7yarab6fzmbb87hdldk66rwh1sx2pv84djr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-2)
                       ("rust-num-traits" ,rust-num-traits-0.2)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serde-repr" ,rust-serde-repr-0.1))))
    (home-page "https://github.com/serenity-rs/serenity")
    (synopsis
     "Rust library for (de)serializing Discord Voice API gateway messages.")
    (description
     "This package provides a Rust library for (de)serializing Discord Voice API
gateway messages.")
    (license license:isc)))

(define-public rust-serde-derive-1
  (package
    (name "rust-serde-derive")
    (version "1.0.204")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serde_derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "08p25262mbmhsr2cg0508d5b1wvljj956rvpg0v3qgg6gc8pxkg0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://serde.rs")
    (synopsis "Macros 1.1 implementation of #[derive(Serialize, Deserialize)]")
    (description
     "This package provides Macros 1.1 implementation of #[derive(Serialize, Deserialize)].")
    (license (list license:expat license:asl2.0))))

(define-public rust-serde-cow-0.1
  (package
    (name "rust-serde-cow")
    (version "0.1.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serde_cow" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1nmplkb2bvk7iqijh01856b89h4783inajxmb8jxxgwnf7nbnyqy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/GnomedDev/serde_cow")
    (synopsis "library with more efficent serde deserializations for Cow.")
    (description
     "This package provides a library with more efficent serde deserializations for
Cow.")
    (license license:expat)))

(define-public rust-secrecy-0.8
  (package
    (name "rust-secrecy")
    (version "0.8.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "secrecy" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "07p9h2bpkkg61f1fzzdqqbf74kwv1gg095r1cdmjzzbcl17cblcv"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytes" ,rust-bytes-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://github.com/iqlusioninc/crates/")
    (synopsis
     "Wrapper types and traits for secret management which help ensure
they aren't accidentally copied, logged, or otherwise exposed
(as much as possible), and also ensure secrets are securely wiped
from memory when dropped.")
    (description
     "This package provides Wrapper types and traits for secret management which help ensure they aren't
accidentally copied, logged, or otherwise exposed (as much as possible), and
also ensure secrets are securely wiped from memory when dropped.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-tagptr-0.2
  (package
    (name "rust-tagptr")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tagptr" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "05r4mwvlsclx1ayj65hpzjv3dn4wpi8j4xm695vydccf9k7r683v"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/oliver-giersch/tagptr.git")
    (synopsis "Strongly typed atomic and non-atomic tagged pointers")
    (description
     "This package provides Strongly typed atomic and non-atomic tagged pointers.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mini-moka-0.10
  (package
    (name "rust-mini-moka")
    (version "0.10.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mini-moka" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "00yqhslppnrl2a54rrrp03xb65d2knbb1s5yvs3g6qgjcnmxy9f3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-crossbeam-channel" ,rust-crossbeam-channel-0.5)
                       ("rust-crossbeam-utils" ,rust-crossbeam-utils-0.8)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-skeptic" ,rust-skeptic-0.13)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-tagptr" ,rust-tagptr-0.2)
                       ("rust-triomphe" ,rust-triomphe-0.1))))
    (home-page "https://github.com/moka-rs/mini-moka")
    (synopsis "lighter edition of Moka, a fast and concurrent cache library")
    (description
     "This package provides a lighter edition of Moka, a fast and concurrent cache
library.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mime-guess-2
  (package
    (name "rust-mime-guess")
    (version "2.0.5")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mime_guess" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "03jmg3yx6j39mg0kayf7w4a886dl3j15y8zs119zw01ccy74zi7p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-mime" ,rust-mime-0.3)
                       ("rust-unicase" ,rust-unicase-2)
                       ("rust-unicase" ,rust-unicase-2))))
    (home-page "https://github.com/abonander/mime_guess")
    (synopsis
     "simple crate for detection of a file's MIME type by its extension.")
    (description
     "This package provides a simple crate for detection of a file's MIME type by its
extension.")
    (license license:expat)))

(define-public rust-merlin-3
  (package
    (name "rust-merlin")
    (version "3.0.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "merlin" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0z9rh9jlpcs0i0cijbs6pcq26gl4qwz05y7zbnv7h2gwk4kqxhsq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-byteorder" ,rust-byteorder-1)
                       ("rust-hex" ,rust-hex-0.3)
                       ("rust-keccak" ,rust-keccak-0.1)
                       ("rust-rand-core" ,rust-rand-core-0.6)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://docs.rs/merlin")
    (synopsis
     "Composable proof transcripts for public-coin arguments of knowledge")
    (description
     "This package provides Composable proof transcripts for public-coin arguments of knowledge.")
    (license license:expat)))

(define-public rust-ed25519-2
  (package
    (name "rust-ed25519")
    (version "2.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "ed25519" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lydzdf26zbn82g7xfczcac9d7mzm3qgx934ijjrd5hjpjx32m8i"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-pkcs8" ,rust-pkcs8-0.10)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-bytes" ,rust-serde-bytes-0.11)
                       ("rust-signature" ,rust-signature-2)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://github.com/RustCrypto/signatures/tree/master/ed25519")
    (synopsis
     "Edwards Digital Signature Algorithm (EdDSA) over Curve25519 (as specified in RFC 8032)
support library providing signature type definitions and PKCS#8 private key
decoding/encoding support")
    (description
     "This package provides Edwards Digital Signature Algorithm (@code{EdDSA}) over Curve25519 (as specified
in RFC 8032) support library providing signature type definitions and PKCS#8
private key decoding/encoding support.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-fiat-crypto-0.2
  (package
    (name "rust-fiat-crypto")
    (version "0.2.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "fiat-crypto" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "07c1vknddv3ak7w89n85ik0g34nzzpms6yb845vrjnv9m4csbpi8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/mit-plv/fiat-crypto")
    (synopsis "Fiat-crypto generated Rust")
    (description "This package provides Fiat-crypto generated Rust.")
    (license (list license:expat license:asl2.0))))

(define-public rust-curve25519-dalek-derive-0.1
  (package
    (name "rust-curve25519-dalek-derive")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "curve25519-dalek-derive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1cry71xxrr0mcy5my3fb502cwfxy6822k4pm19cwrilrg7hq4s7l"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/dalek-cryptography/curve25519-dalek")
    (synopsis "curve25519-dalek Derives")
    (description "This package provides curve25519-dalek Derives.")
    (license (list license:expat license:asl2.0))))

(define-public rust-curve25519-dalek-4
  (package
    (name "rust-curve25519-dalek")
    (version "4.1.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "curve25519-dalek" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1gmjb9dsknrr8lypmhkyjd67p1arb8mbfamlwxm7vph38my8pywp"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-cpufeatures" ,rust-cpufeatures-0.2)
                       ("rust-curve25519-dalek-derive" ,rust-curve25519-dalek-derive-0.1)
                       ("rust-digest" ,rust-digest-0.10)
                       ("rust-ff" ,rust-ff-0.13)
                       ("rust-fiat-crypto" ,rust-fiat-crypto-0.2)
                       ("rust-group" ,rust-group-0.13)
                       ("rust-rand-core" ,rust-rand-core-0.6)
                       ("rust-rustc-version" ,rust-rustc-version-0.4)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-subtle" ,rust-subtle-2)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://github.com/dalek-cryptography/curve25519-dalek")
    (synopsis
     "pure-Rust implementation of group operations on ristretto255 and Curve25519")
    (description
     "This package provides a pure-Rust implementation of group operations on
ristretto255 and Curve25519.")
    (license license:bsd-3)))

(define-public rust-ed25519-dalek-2
  (package
    (name "rust-ed25519-dalek")
    (version "2.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "ed25519-dalek" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0w88cafwglg9hjizldbmlza0ns3hls81zk1bcih3m5m3h67algaa"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-curve25519-dalek" ,rust-curve25519-dalek-4)
                       ("rust-ed25519" ,rust-ed25519-2)
                       ("rust-merlin" ,rust-merlin-3)
                       ("rust-rand-core" ,rust-rand-core-0.6)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-signature" ,rust-signature-2)
                       ("rust-subtle" ,rust-subtle-2)
                       ("rust-zeroize" ,rust-zeroize-1))))
    (home-page "https://github.com/dalek-cryptography/curve25519-dalek")
    (synopsis
     "Fast and efficient ed25519 EdDSA key generations, signing, and verification in pure Rust")
    (description
     "This package provides Fast and efficient ed25519 @code{EdDSA} key generations, signing, and
verification in pure Rust.")
    (license license:bsd-3)))

(define-public rust-command-attr-0.5
  (package
    (name "rust-command-attr")
    (version "0.5.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "command_attr" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0cbnvjdyhfc47rim5sgc1zkrvd0287wd0wng7y70vwz6kxz8vnl8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "")
    (synopsis
     "Procedural macros for command creation for the Serenity library")
    (description
     "This package provides Procedural macros for command creation for the Serenity library.")
    (license license:isc)))

(define-public rust-bytes-1
  (package
    (name "rust-bytes")
    (version "1.7.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "bytes" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1jgss90klcbgqsafnjlzymy8gdzw785z7qsf6sp2p0a3bhfvx8pw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/tokio-rs/bytes")
    (synopsis "Types and traits for working with bytes")
    (description
     "This package provides Types and traits for working with bytes.")
    (license license:expat)))

(define-public rust-bitflags-2
  (package
    (name "rust-bitflags")
    (version "2.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "bitflags" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1pkidwzn3hnxlsl8zizh0bncgbjnw7c41cx7bby26ncbzmiznj5h"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arbitrary" ,rust-arbitrary-1)
                       ("rust-bytemuck" ,rust-bytemuck-1)
                       ("rust-compiler-builtins" ,rust-compiler-builtins-0.1)
                       ("rust-rustc-std-workspace-core" ,rust-rustc-std-workspace-core-1)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/bitflags/bitflags")
    (synopsis "macro to generate structures which behave like bitflags.")
    (description
     "This package provides a macro to generate structures which behave like bitflags.")
    (license (list license:expat license:asl2.0))))

(define-public rust-base64-0.22
  (package
    (name "rust-base64")
    (version "0.22.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "base64" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1imqzgh7bxcikp5vx3shqvw9j09g9ly0xr0jma0q66i52r7jbcvj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/marshallpierce/rust-base64")
    (synopsis "encodes and decodes base64 as bytes or utf8")
    (description
     "This package provides encodes and decodes base64 as bytes or utf8.")
    (license (list license:expat license:asl2.0))))

(define-public rust-serenity-0.12
  (package
    (name "rust-serenity")
    (version "0.12.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "serenity" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1b1lblimgn4w9nc78w2gl6scrywvi0sxkcdcppssil4jcl8082l8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-base64" ,rust-base64-0.22)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-command-attr" ,rust-command-attr-0.5)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-ed25519-dalek" ,rust-ed25519-dalek-2)
                       ("rust-flate2" ,rust-flate2-1)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-fxhash" ,rust-fxhash-0.2)
                       ("rust-levenshtein" ,rust-levenshtein-1)
                       ("rust-mime-guess" ,rust-mime-guess-2)
                       ("rust-mini-moka" ,rust-mini-moka-0.10)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-percent-encoding" ,rust-percent-encoding-2)
                       ("rust-reqwest" ,rust-reqwest-0.11)
                       ("rust-secrecy" ,rust-secrecy-0.8)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-cow" ,rust-serde-cow-0.1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serenity-voice-model" ,rust-serenity-voice-model-0.2)
                       ("rust-simd-json" ,rust-simd-json-0.13)
                       ("rust-static-assertions" ,rust-static-assertions-1)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-tungstenite" ,rust-tokio-tungstenite-0.21)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-typemap-rev" ,rust-typemap-rev-0.3)
                       ("rust-typesize" ,rust-typesize-0.1)
                       ("rust-url" ,rust-url-2)
                       ("rust-uwl" ,rust-uwl-0.6))))
    (home-page "https://github.com/serenity-rs/serenity")
    (synopsis "Rust library for the Discord API.")
    (description "This package provides a Rust library for the Discord API.")
    (license license:isc)))

(define-public rust-rusty-pool-0.7
  (package
    (name "rust-rusty-pool")
    (version "0.7.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rusty_pool" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1zvn93786m80lpfawyl7rzr8d8y77y4bh17a2yddhrny43dnrlsf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-crossbeam-channel" ,rust-crossbeam-channel-0.5)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-futures-channel" ,rust-futures-channel-0.3)
                       ("rust-futures-executor" ,rust-futures-executor-0.3)
                       ("rust-num-cpus" ,rust-num-cpus-1))))
    (home-page "https://github.com/robinfriedli/rusty_pool")
    (synopsis
     "Self growing / shrinking `ThreadPool` implementation based on crossbeam's multi-producer multi-consumer channels that enables awaiting the result of a task and offers async support")
    (description
     "This package provides Self growing / shrinking `@code{ThreadPool`} implementation based on crossbeam's
multi-producer multi-consumer channels that enables awaiting the result of a
task and offers async support.")
    (license license:asl2.0)))

(define-public rust-transpose-0.2
  (package
    (name "rust-transpose")
    (version "0.2.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "transpose" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0zp74v7jrjg4jr654dncdj6hqvacicsywyhc62jawgxwhvnimmhs"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-num-integer" ,rust-num-integer-0.1)
                       ("rust-strength-reduce" ,rust-strength-reduce-0.2))))
    (home-page "https://github.com/ejmahler/transpose")
    (synopsis "Utility for transposing multi-dimensional data")
    (description
     "This package provides Utility for transposing multi-dimensional data.")
    (license (list license:expat license:asl2.0))))

(define-public rust-strength-reduce-0.2
  (package
    (name "rust-strength-reduce")
    (version "0.2.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "strength_reduce" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "10jdq9dijjdkb20wg1dmwg447rnj37jbq0mwvbadvqi2gys5x2gy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "http://github.com/ejmahler/strength_reduce")
    (synopsis "Faster integer division and modulus operations")
    (description
     "This package provides Faster integer division and modulus operations.")
    (license (list license:expat license:asl2.0))))

(define-public rust-primal-check-0.3
  (package
    (name "rust-primal-check")
    (version "0.3.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "primal-check" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "025xnak4rhkwa4h970bjb3cvp2k853wviyr84n8gjfhy65dqj3fw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-num-integer" ,rust-num-integer-0.1))))
    (home-page "https://github.com/huonw/primal")
    (synopsis "Fast standalone primality testing.")
    (description "This package provides Fast standalone primality testing.")
    (license (list license:expat license:asl2.0))))

(define-public rust-rustfft-6
  (package
    (name "rust-rustfft")
    (version "6.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rustfft" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "11hx83yr2h2jszkba9qhq2d08q9i5rsashq62rfhqvahpihnb023"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-num-complex" ,rust-num-complex-0.4)
                       ("rust-num-integer" ,rust-num-integer-0.1)
                       ("rust-num-traits" ,rust-num-traits-0.2)
                       ("rust-primal-check" ,rust-primal-check-0.3)
                       ("rust-strength-reduce" ,rust-strength-reduce-0.2)
                       ("rust-transpose" ,rust-transpose-0.2)
                       ("rust-version-check" ,rust-version-check-0.9))))
    (home-page "https://github.com/ejmahler/RustFFT")
    (synopsis "High-performance FFT library written in pure Rust")
    (description
     "This package provides High-performance FFT library written in pure Rust.")
    (license (list license:expat license:asl2.0))))

(define-public rust-realfft-3
  (package
    (name "rust-realfft")
    (version "3.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "realfft" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0w854bh695cxhrng7jz3sip65hpgjq92amml8wsrd06xbiz9ygcm"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-rustfft" ,rust-rustfft-6))))
    (home-page "https://github.com/HEnquist/realfft")
    (synopsis
     "Real-to-complex forward FFT and complex-to-real inverse FFT for Rust")
    (description
     "This package provides Real-to-complex forward FFT and complex-to-real inverse FFT for Rust.")
    (license license:expat)))

(define-public rust-rubato-0.15
  (package
    (name "rust-rubato")
    (version "0.15.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rubato" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0d7281cn4d5pyni7r60w16pqg3vx6b0mp0izxw4a8abxdr48pldm"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-log" ,rust-log-0.4)
                       ("rust-num-complex" ,rust-num-complex-0.4)
                       ("rust-num-integer" ,rust-num-integer-0.1)
                       ("rust-num-traits" ,rust-num-traits-0.2)
                       ("rust-realfft" ,rust-realfft-3))))
    (home-page "https://github.com/HEnquist/rubato")
    (synopsis "Asynchronous resampling library intended for audio data")
    (description
     "This package provides Asynchronous resampling library intended for audio data.")
    (license license:expat)))

(define-public rust-ringbuf-0.4
  (package
    (name "rust-ringbuf")
    (version "0.4.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "ringbuf" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0dinbij3k77sb4hf4d3f0sfjav3yrw5gyga9jhr2wgdwcp4f8raw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-crossbeam-utils" ,rust-crossbeam-utils-0.8))))
    (home-page "https://gitlab.com/agerasev/ringbuf.git")
    (synopsis
     "Lock-free SPSC FIFO ring buffer with direct access to inner data")
    (description
     "This package provides Lock-free SPSC FIFO ring buffer with direct access to inner data.")
    (license (list license:expat license:asl2.0))))

(define-public rust-nohash-hasher-0.2
  (package
    (name "rust-nohash-hasher")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "nohash-hasher" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lf4p6k01w4wm7zn4grnihzj8s7zd5qczjmzng7wviwxawih5x9b"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/paritytech/nohash-hasher")
    (synopsis
     "An implementation of `std::hash::Hasher` which does not hash at all")
    (description
     "This package provides An implementation of `std::hash::Hasher` which does not hash at all.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-flume-0.11
  (package
    (name "rust-flume")
    (version "0.11.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "flume" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "10girdbqn77wi802pdh55lwbmymy437k7kklnvj12aaiwaflbb2m"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-sink" ,rust-futures-sink-0.3)
                       ("rust-nanorand" ,rust-nanorand-0.7)
                       ("rust-spin" ,rust-spin-0.9))))
    (home-page "https://github.com/zesterer/flume")
    (synopsis "blazingly fast multi-producer channel")
    (description
     "This package provides a blazingly fast multi-producer channel.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-no-std-net-0.6
  (package
    (name "rust-no-std-net")
    (version "0.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "no-std-net" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ravflgyh0q2142gjdz9iav5yqci3ga7gbnk4mmfcnqkrq54lya3"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/dunmatt/no-std-net")
    (synopsis "Rust's std::net... without the 'std'")
    (description "This package provides Rust's std::net...  without the std'.")
    (license license:expat)))

(define-public rust-pnet-base-0.34
  (package
    (name "rust-pnet-base")
    (version "0.34.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "pnet_base" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0k2z3jh6vw99bwl0ckpsd142n4yiscza1bmj3b86i2xk7bxzck7y"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-no-std-net" ,rust-no-std-net-0.6)
                       ("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/libpnet/libpnet")
    (synopsis "Fundamental base types and code used by pnet")
    (description
     "This package provides Fundamental base types and code used by pnet.")
    (license (list license:expat license:asl2.0))))

(define-public rust-pnet-macros-support-0.34
  (package
    (name "rust-pnet-macros-support")
    (version "0.34.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "pnet_macros_support" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0mhg0dzdxg6lkpx3z60c6nc39dkq9jz1n8hgmf77zlsb5yvjbagf"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-pnet-base" ,rust-pnet-base-0.34))))
    (home-page "https://github.com/libpnet/libpnet")
    (synopsis "Support library for libpnet_macros")
    (description "This package provides Support library for libpnet_macros.")
    (license (list license:expat license:asl2.0))))

(define-public rust-pnet-macros-0.34
  (package
    (name "rust-pnet-macros")
    (version "0.34.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "pnet_macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "010qhwkw597pzmiqmphnf8fl11pwlp5sa2nai90a017fkr4ig2v8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/libpnet/libpnet")
    (synopsis "Automatic bit manipulation for binary data formats")
    (description
     "This package provides Automatic bit manipulation for binary data formats.")
    (license (list license:expat license:asl2.0))))

(define-public rust-discortp-0.6
  (package
    (name "rust-discortp")
    (version "0.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "discortp" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0p9zgprysjvv2a1dzbvzc7128156zwiq39izc5a1fdjxl4438yaw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-pnet-macros" ,rust-pnet-macros-0.34)
                       ("rust-pnet-macros-support" ,rust-pnet-macros-support-0.34))))
    (home-page "https://github.com/FelixMcFelix/discortp")
    (synopsis
     "Lightweight, flexible Real-time Transport Protocol (RTP) parsing library")
    (description
     "This package provides Lightweight, flexible Real-time Transport Protocol (RTP) parsing library.")
    (license license:isc)))

(define-public rust-audiopus-sys-0.2
  (package
    (name "rust-audiopus-sys")
    (version "0.2.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "audiopus_sys" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lc6kzdw65kbgqaghig99f8642k2ikl5imk56q1lw1m28qallcb2"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bindgen" ,rust-bindgen-0.58)
                       ("rust-cmake" ,rust-cmake-0.1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-pkg-config" ,rust-pkg-config-0.3))))
    (home-page "https://github.com/lakelezz/audiopus_sys.git")
    (synopsis
     "FFI-Binding to Opus, dynamically or statically linked for Windows and UNIX")
    (description
     "This package provides FFI-Binding to Opus, dynamically or statically linked for Windows and UNIX.")
    (license license:isc)))

(define-public rust-audiopus-0.3
  (package
    (name "rust-audiopus")
    (version "0.3.0-rc.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "audiopus" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0m9r8zk3n7r4x1p2fsmy6gn2axrd2bdyai7mb4yxxinpaq7fnmdb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-audiopus-sys" ,rust-audiopus-sys-0.2))))
    (home-page "https://github.com/lakelezz/audiopus.git")
    (synopsis "High-level binding of the Opus Codec library")
    (description
     "This package provides High-level binding of the Opus Codec library.")
    (license license:isc)))

(define-public rust-songbird-0.4
  (package
    (name "rust-songbird")
    (version "0.4.3")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "songbird" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1lchmiwc6q8brw484a7cxiw4g8r8irc3iny0qj1zb17hz61d339k"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-audiopus" ,rust-audiopus-0.3)
                       ("rust-byteorder" ,rust-byteorder-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-crypto-secretbox" ,rust-crypto-secretbox-0.1)
                       ("rust-dashmap" ,rust-dashmap-5)
                       ("rust-derivative" ,rust-derivative-2)
                       ("rust-discortp" ,rust-discortp-0.6)
                       ("rust-flume" ,rust-flume-0.11)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-nohash-hasher" ,rust-nohash-hasher-0.2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-pin-project" ,rust-pin-project-1)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-reqwest" ,rust-reqwest-0.11)
                       ("rust-ringbuf" ,rust-ringbuf-0.4)
                       ("rust-rubato" ,rust-rubato-0.15)
                       ("rust-rusty-pool" ,rust-rusty-pool-0.7)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-aux" ,rust-serde-aux-4)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serenity" ,rust-serenity-0.12)
                       ("rust-serenity-voice-model" ,rust-serenity-voice-model-0.2)
                       ("rust-simd-json" ,rust-simd-json-0.13)
                       ("rust-socket2" ,rust-socket2-0.5)
                       ("rust-stream-lib" ,rust-stream-lib-0.4)
                       ("rust-streamcatcher" ,rust-streamcatcher-1)
                       ("rust-symphonia" ,rust-symphonia-0.5)
                       ("rust-symphonia-core" ,rust-symphonia-core-0.5)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-tungstenite" ,rust-tokio-tungstenite-0.21)
                       ("rust-tokio-util" ,rust-tokio-util-0.7)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-tracing-futures" ,rust-tracing-futures-0.2)
                       ("rust-twilight-gateway" ,rust-twilight-gateway-0.15)
                       ("rust-twilight-model" ,rust-twilight-model-0.15)
                       ("rust-typemap-rev" ,rust-typemap-rev-0.3)
                       ("rust-url" ,rust-url-2)
                       ("rust-uuid" ,rust-uuid-1))
       #:cargo-development-inputs (("rust-byteorder" ,rust-byteorder-1)
                                   ("rust-criterion" ,rust-criterion-0.5)
                                   ("rust-ntest" ,rust-ntest-0.9)
                                   ("rust-symphonia" ,rust-symphonia-0.5)
                                   ("rust-tokio" ,rust-tokio-1))))
    (home-page "https://github.com/serenity-rs/songbird")
    (synopsis "An async Rust library for the Discord voice API")
    (description
     "This package provides An async Rust library for the Discord voice API.")
    (license license:isc)))

(define-public rust-flume-0.11
  (package
    (name "rust-flume")
    (version "0.11.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "flume" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "10girdbqn77wi802pdh55lwbmymy437k7kklnvj12aaiwaflbb2m"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-sink" ,rust-futures-sink-0.3)
                       ("rust-nanorand" ,rust-nanorand-0.7)
                       ("rust-spin" ,rust-spin-0.9))))
    (home-page "https://github.com/zesterer/flume")
    (synopsis "blazingly fast multi-producer channel")
    (description
     "This package provides a blazingly fast multi-producer channel.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-sqlx-sqlite-0.7
  (package
    (name "rust-sqlx-sqlite")
    (version "0.7.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sqlx-sqlite" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ap0bb2hazbrdgd7mhnckdg9xcchx0k094di9gnhpnhlhh5fyi5j"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-atoi" ,rust-atoi-2)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-flume" ,rust-flume-0.11)
                       ("rust-futures-channel" ,rust-futures-channel-0.3)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-executor" ,rust-futures-executor-0.3)
                       ("rust-futures-intrusive" ,rust-futures-intrusive-0.5)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-libsqlite3-sys" ,rust-libsqlite3-sys-0.27)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-percent-encoding" ,rust-percent-encoding-2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-sqlx-core" ,rust-sqlx-core-0.7)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-url" ,rust-url-2)
                       ("rust-urlencoding" ,rust-urlencoding-2)
                       ("rust-uuid" ,rust-uuid-1))))
    (home-page "https://github.com/launchbadge/sqlx")
    (synopsis
     "SQLite driver implementation for SQLx. Not for direct use; see the `sqlx` crate for details")
    (description
     "This package provides SQLite driver implementation for SQLx.  Not for direct use; see the `sqlx` crate
for details.")
    (license (list license:expat license:asl2.0))))

(define-public rust-sqlx-postgres-0.7
  (package
    (name "rust-sqlx-postgres")
    (version "0.7.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sqlx-postgres" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0zjp30wj4n2f25dnb32vsg6jfpa3gw6dmfd0i5pr4kw91fw4x0kw"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-atoi" ,rust-atoi-2)
                       ("rust-base64" ,rust-base64-0.21)
                       ("rust-bigdecimal" ,rust-bigdecimal-0.3)
                       ("rust-bit-vec" ,rust-bit-vec-0.6)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-byteorder" ,rust-byteorder-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-crc" ,rust-crc-3)
                       ("rust-dotenvy" ,rust-dotenvy-0.15)
                       ("rust-etcetera" ,rust-etcetera-0.8)
                       ("rust-futures-channel" ,rust-futures-channel-0.3)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-io" ,rust-futures-io-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-hex" ,rust-hex-0.4)
                       ("rust-hkdf" ,rust-hkdf-0.12)
                       ("rust-hmac" ,rust-hmac-0.12)
                       ("rust-home" ,rust-home-0.5)
                       ("rust-ipnetwork" ,rust-ipnetwork-0.20)
                       ("rust-itoa" ,rust-itoa-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-mac-address" ,rust-mac-address-1)
                       ("rust-md-5" ,rust-md-5-0.10)
                       ("rust-memchr" ,rust-memchr-2)
                       ("rust-num-bigint" ,rust-num-bigint-0.4)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-rust-decimal" ,rust-rust-decimal-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-sqlx-core" ,rust-sqlx-core-0.7)
                       ("rust-stringprep" ,rust-stringprep-0.1)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-uuid" ,rust-uuid-1)
                       ("rust-whoami" ,rust-whoami-1))))
    (home-page "https://github.com/launchbadge/sqlx")
    (synopsis
     "PostgreSQL driver implementation for SQLx. Not for direct use; see the `sqlx` crate for details")
    (description
     "This package provides @code{PostgreSQL} driver implementation for SQLx.  Not for direct use; see the
`sqlx` crate for details.")
    (license (list license:expat license:asl2.0))))

(define-public rust-sqlx-mysql-0.7
  (package
    (name "rust-sqlx-mysql")
    (version "0.7.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sqlx-mysql" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "066lxhb80xgb8r5m2yy3a7ydjvp0b6wsk9s7whwfa83d46817lqy"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-atoi" ,rust-atoi-2)
                       ("rust-base64" ,rust-base64-0.21)
                       ("rust-bigdecimal" ,rust-bigdecimal-0.3)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-byteorder" ,rust-byteorder-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-crc" ,rust-crc-3)
                       ("rust-digest" ,rust-digest-0.10)
                       ("rust-dotenvy" ,rust-dotenvy-0.15)
                       ("rust-either" ,rust-either-1)
                       ("rust-futures-channel" ,rust-futures-channel-0.3)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-io" ,rust-futures-io-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-generic-array" ,rust-generic-array-0.14)
                       ("rust-hex" ,rust-hex-0.4)
                       ("rust-hkdf" ,rust-hkdf-0.12)
                       ("rust-hmac" ,rust-hmac-0.12)
                       ("rust-itoa" ,rust-itoa-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-md-5" ,rust-md-5-0.10)
                       ("rust-memchr" ,rust-memchr-2)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-percent-encoding" ,rust-percent-encoding-2)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-rsa" ,rust-rsa-0.9)
                       ("rust-rust-decimal" ,rust-rust-decimal-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-sha1" ,rust-sha1-0.10)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-sqlx-core" ,rust-sqlx-core-0.7)
                       ("rust-stringprep" ,rust-stringprep-0.1)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-uuid" ,rust-uuid-1)
                       ("rust-whoami" ,rust-whoami-1))))
    (home-page "https://github.com/launchbadge/sqlx")
    (synopsis
     "MySQL driver implementation for SQLx. Not for direct use; see the `sqlx` crate for details")
    (description
     "This package provides @code{MySQL} driver implementation for SQLx.  Not for direct use; see the `sqlx`
crate for details.")
    (license (list license:expat license:asl2.0))))

(define-public rust-sqlx-macros-core-0.7
  (package
    (name "rust-sqlx-macros-core")
    (version "0.7.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sqlx-macros-core" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1j7k0fw7n6pgabqnj6cbp8s3rmd3yvqr4chjj878cvd1m99yycsq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-async-std" ,rust-async-std-1)
                       ("rust-dotenvy" ,rust-dotenvy-0.15)
                       ("rust-either" ,rust-either-1)
                       ("rust-heck" ,rust-heck-0.4)
                       ("rust-hex" ,rust-hex-0.4)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-sqlx-core" ,rust-sqlx-core-0.7)
                       ("rust-sqlx-mysql" ,rust-sqlx-mysql-0.7)
                       ("rust-sqlx-postgres" ,rust-sqlx-postgres-0.7)
                       ("rust-sqlx-sqlite" ,rust-sqlx-sqlite-0.7)
                       ("rust-syn" ,rust-syn-1)
                       ("rust-tempfile" ,rust-tempfile-3)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-url" ,rust-url-2))))
    (home-page "https://github.com/launchbadge/sqlx")
    (synopsis
     "Macro support core for SQLx, the Rust SQL toolkit. Not intended to be used directly")
    (description
     "This package provides Macro support core for SQLx, the Rust SQL toolkit.  Not intended to be used
directly.")
    (license (list license:expat license:asl2.0))))

(define-public rust-sqlx-macros-0.7
  (package
    (name "rust-sqlx-macros")
    (version "0.7.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sqlx-macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "09rih250868nfkax022y5dyk24a7qfw6scjy3sgalbzb8lihx92f"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-sqlx-core" ,rust-sqlx-core-0.7)
                       ("rust-sqlx-macros-core" ,rust-sqlx-macros-core-0.7)
                       ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/launchbadge/sqlx")
    (synopsis
     "Macros for SQLx, the rust SQL toolkit. Not intended to be used directly")
    (description
     "This package provides Macros for SQLx, the rust SQL toolkit.  Not intended to be used directly.")
    (license (list license:expat license:asl2.0))))

(define-public rust-sqlformat-0.2
  (package
    (name "rust-sqlformat")
    (version "0.2.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sqlformat" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "07vwxjfcbdnbzsg3683mshjc245rr4k8j9b6zvqmbk0q8dry75gq"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-nom" ,rust-nom-7)
                       ("rust-unicode-categories" ,rust-unicode-categories-0.1))))
    (home-page "https://github.com/shssoichiro/sqlformat-rs")
    (synopsis "Formats whitespace in a SQL string to make it easier to read")
    (description
     "This package provides Formats whitespace in a SQL string to make it easier to read.")
    (license (list license:expat license:asl2.0))))

(define-public rust-is-terminal-0.4
  (package
    (name "rust-is-terminal")
    (version "0.4.12")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "is-terminal" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "12vk6g0f94zlxl6mdh5gc4jdjb469n9k9s7y3vb0iml05gpzagzj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-hermit-abi" ,rust-hermit-abi-0.3)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-windows-sys" ,rust-windows-sys-0.52))))
    (home-page "https://github.com/sunfishcode/is-terminal")
    (synopsis "Test whether a given stream is a terminal")
    (description
     "This package provides Test whether a given stream is a terminal.")
    (license license:expat)))

(define-public rust-yansi-1
  (package
    (name "rust-yansi")
    (version "1.0.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "yansi" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0jdh55jyv0dpd38ij4qh60zglbw9aa8wafqai6m0wa7xaxk3mrfg"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-is-terminal" ,rust-is-terminal-0.4))))
    (home-page "https://github.com/SergioBenitez/yansi")
    (synopsis "dead simple ANSI terminal color painting library.")
    (description
     "This package provides a dead simple ANSI terminal color painting library.")
    (license (list license:expat license:asl2.0))))

(define-public rust-state-0.6
  (package
    (name "rust-state")
    (version "0.6.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "state" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1n3n2h324h1y5zhaajh6kplvzfvg1l6hsr8siggmf4yq8m24m31b"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-loom" ,rust-loom-0.5))))
    (home-page "https://github.com/SergioBenitez/state")
    (synopsis
     "library for safe and effortless global and thread-local state management.")
    (description
     "This package provides a library for safe and effortless global and thread-local
state management.")
    (license (list license:expat license:asl2.0))))

(define-public rust-stable-pattern-0.1
  (package
    (name "rust-stable-pattern")
    (version "0.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "stable-pattern" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0i8hq82vm82mqj02qqcsd7caibrih7x5w3a1xpm8hpv30261cr25"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-memchr" ,rust-memchr-2))))
    (home-page "https://github.com/SergioBenitez/stable-pattern")
    (synopsis "Stable port of std::str::Pattern and friends")
    (description
     "This package provides Stable port of std::str::Pattern and friends.")
    (license (list license:expat license:asl2.0))))

(define-public rust-rocket-http-0.5
  (package
    (name "rust-rocket-http")
    (version "0.5.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rocket_http" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1fc2z0a7zhmf8rh7s1dwdmmkjmq7qiivsi6027v6ac7f41d92x72"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cookie" ,rust-cookie-0.18)
                       ("rust-either" ,rust-either-1)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-http" ,rust-http-0.2)
                       ("rust-hyper" ,rust-hyper-0.14)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-memchr" ,rust-memchr-2)
                       ("rust-pear" ,rust-pear-0.2)
                       ("rust-percent-encoding" ,rust-percent-encoding-2)
                       ("rust-pin-project-lite" ,rust-pin-project-lite-0.2)
                       ("rust-ref-cast" ,rust-ref-cast-1)
                       ("rust-rustls" ,rust-rustls-0.21)
                       ("rust-rustls-pemfile" ,rust-rustls-pemfile-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-stable-pattern" ,rust-stable-pattern-0.1)
                       ("rust-state" ,rust-state-0.6)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-rustls" ,rust-tokio-rustls-0.24)
                       ("rust-uncased" ,rust-uncased-0.9)
                       ("rust-uuid" ,rust-uuid-1)
                       ("rust-x509-parser" ,rust-x509-parser-0.13))))
    (home-page "https://rocket.rs")
    (synopsis
     "Types, traits, and parsers for HTTP requests, responses, and headers.")
    (description
     "This package provides Types, traits, and parsers for HTTP requests, responses, and headers.")
    (license (list license:expat license:asl2.0))))

(define-public rust-rocket-codegen-0.5
  (package
    (name "rust-rocket-codegen")
    (version "0.5.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rocket_codegen" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0inxw7nzr52sabwpz83cz5rh1a0mg32cg7w7ih8715qsxkbk4pap"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-devise" ,rust-devise-0.4)
                       ("rust-glob" ,rust-glob-0.3)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-rocket-http" ,rust-rocket-http-0.5)
                       ("rust-syn" ,rust-syn-2)
                       ("rust-unicode-xid" ,rust-unicode-xid-0.2)
                       ("rust-version-check" ,rust-version-check-0.9))))
    (home-page "https://rocket.rs")
    (synopsis "Procedural macros for the Rocket web framework")
    (description
     "This package provides Procedural macros for the Rocket web framework.")
    (license (list license:expat license:asl2.0))))

(define-public rust-multer-3
  (package
    (name "rust-multer")
    (version "3.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "multer" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0jr2snfay5fjz50yvdja4vbnddlj1iriiqjym88pbj3daiv7gs43"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bytes" ,rust-bytes-1)
                       ("rust-encoding-rs" ,rust-encoding-rs-0.8)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-http" ,rust-http-1)
                       ("rust-httparse" ,rust-httparse-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-memchr" ,rust-memchr-2)
                       ("rust-mime" ,rust-mime-0.3)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-spin" ,rust-spin-0.9)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-util" ,rust-tokio-util-0.7)
                       ("rust-version-check" ,rust-version-check-0.9))))
    (home-page "https://github.com/rwf2/multer")
    (synopsis "An async parser for `multipart/form-data` content-type in Rust")
    (description
     "This package provides An async parser for `multipart/form-data` content-type in Rust.")
    (license license:expat)))

(define-public rust-pear-codegen-0.2
  (package
    (name "rust-pear-codegen")
    (version "0.2.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "pear_codegen" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0izijffdd2xs762497mk0xr7xwmyw62dzdqjz12v70n0bnc5pasb"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-proc-macro2-diagnostics" ,rust-proc-macro2-diagnostics-0.10)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://github.com/SergioBenitez/Pear")
    (synopsis "(codegen) pear is a fruit.")
    (description "This package provides a (codegen) pear is a fruit.")
    (license (list license:expat license:asl2.0))))

(define-public rust-inlinable-string-0.1
  (package
    (name "rust-inlinable-string")
    (version "0.1.15")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "inlinable_string" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ysjci8yfvxgf51z0ny2nnwhxrclhmb3vbngin8v4bznhr3ybyn8"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/fitzgen/inlinable_string")
    (synopsis
     "The `inlinable_string` crate provides the `InlinableString` type -- an owned, grow-able UTF-8 string that stores small strings inline and avoids heap-allocation -- and the `StringExt` trait which abstracts string operations over both `std::string::String` and `InlinableString` (or even your own custom string type)")
    (description
     "This package provides The `inlinable_string` crate provides the `@code{InlinableString`} type -- an
owned, grow-able UTF-8 string that stores small strings inline and avoids
heap-allocation -- and the `@code{StringExt`} trait which abstracts string
operations over both `std::string::String` and `@code{InlinableString`} (or even
your own custom string type).")
    (license (list license:asl2.0 license:expat))))

(define-public rust-pear-0.2
  (package
    (name "rust-pear")
    (version "0.2.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "pear" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0rxlyizzaqq6lswgyfdxjxd3dyb1jfml9gwfpbx5g1j8rq0amvmx"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-inlinable-string" ,rust-inlinable-string-0.1)
                       ("rust-pear-codegen" ,rust-pear-codegen-0.2)
                       ("rust-yansi" ,rust-yansi-1))))
    (home-page "https://github.com/SergioBenitez/Pear")
    (synopsis "pear is a fruit.")
    (description "This package provides a pear is a fruit.")
    (license (list license:expat license:asl2.0))))

(define-public rust-figment-0.10
  (package
    (name "rust-figment")
    (version "0.10.19")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "figment" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ww9sxdzjj2i80w7bq0kllnymjyrfb1cdx2h70ap5wqcdga1rc4c"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-atomic" ,rust-atomic-0.6)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-pear" ,rust-pear-0.2)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-serde-yaml" ,rust-serde-yaml-0.9)
                       ("rust-tempfile" ,rust-tempfile-3)
                       ("rust-toml" ,rust-toml-0.8)
                       ("rust-uncased" ,rust-uncased-0.9)
                       ("rust-version-check" ,rust-version-check-0.9))))
    (home-page "https://github.com/SergioBenitez/Figment")
    (synopsis "configuration library so con-free, it's unreal.")
    (description
     "This package provides a configuration library so con-free, it's unreal.")
    (license (list license:expat license:asl2.0))))

(define-public rust-rocket-0.5
  (package
    (name "rust-rocket")
    (version "0.5.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rocket" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0gypf9z6s0kshv33qq1vf16raw8xnr1p03ii0kfgf7d3jrr905m5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-async-stream" ,rust-async-stream-0.3)
                       ("rust-async-trait" ,rust-async-trait-0.1)
                       ("rust-atomic" ,rust-atomic-0.5)
                       ("rust-binascii" ,rust-binascii-0.1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-either" ,rust-either-1)
                       ("rust-figment" ,rust-figment-0.10)
                       ("rust-futures" ,rust-futures-0.3)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-memchr" ,rust-memchr-2)
                       ("rust-multer" ,rust-multer-3)
                       ("rust-num-cpus" ,rust-num-cpus-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-pin-project-lite" ,rust-pin-project-lite-0.2)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-ref-cast" ,rust-ref-cast-1)
                       ("rust-rmp-serde" ,rust-rmp-serde-1)
                       ("rust-rocket-codegen" ,rust-rocket-codegen-0.5)
                       ("rust-rocket-http" ,rust-rocket-http-0.5)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-state" ,rust-state-0.6)
                       ("rust-tempfile" ,rust-tempfile-3)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-stream" ,rust-tokio-stream-0.1)
                       ("rust-tokio-util" ,rust-tokio-util-0.7)
                       ("rust-ubyte" ,rust-ubyte-0.10)
                       ("rust-uuid" ,rust-uuid-1)
                       ("rust-version-check" ,rust-version-check-0.9)
                       ("rust-yansi" ,rust-yansi-1))))
    (home-page "https://rocket.rs")
    (synopsis
     "Web framework with a focus on usability, security, extensibility, and speed.")
    (description
     "This package provides Web framework with a focus on usability, security, extensibility, and speed.")
    (license (list license:expat license:asl2.0))))

(define-public rust-pq-src-0.3
  (package
    (name "rust-pq-src")
    (version "0.3.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "pq-src" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1sb2gnzs4y3a31ql099ryaprsx4f776m66r648yxmgarasdbqxf0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1)
                       ("rust-openssl-sys" ,rust-openssl-sys-0.9))))
    (home-page "https://github.com/sgrif/pq-sys")
    (synopsis "Bundled version of libpq")
    (description "This package provides Bundled version of libpq.")
    (license #f)))

(define-public rust-openssl-src-300
  (package
    (name "rust-openssl-src")
    (version "300.3.1+3.3.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "openssl-src" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "14cbc0i3ly3msl7bkhj3rrnlv4g1m0qbswxxzcvz26x888yranbj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cc" ,rust-cc-1))))
    (home-page "https://github.com/alexcrichton/openssl-src-rs")
    (synopsis "Source of OpenSSL and logic to build it.")
    (description
     "This package provides Source of @code{OpenSSL} and logic to build it.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mysqlclient-src-0.1
  (package
    (name "rust-mysqlclient-src")
    (version "0.1.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mysqlclient-src" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1zjcdcbax9x5ysdiw41pbkzpg08w9f9alwcy80mrfgh4yiy34zxx"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-cmake" ,rust-cmake-0.1)
                       ("rust-link-cplusplus" ,rust-link-cplusplus-1)
                       ("rust-openssl-src" ,rust-openssl-src-300)
                       ("rust-openssl-sys" ,rust-openssl-sys-0.9))))
    (home-page "https://github.com/sgrif/mysqlclient-sys")
    (synopsis "Bundled version of libmysqlclient")
    (description "This package provides Bundled version of libmysqlclient.")
    (license license:gpl2)))

(define-public rust-heck-0.5
  (package
    (name "rust-heck")
    (version "0.5.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "heck" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1sjmpsdl8czyh9ywl3qcsfsq9a307dg4ni2vnlwgnzzqhc4y0113"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/withoutboats/heck")
    (synopsis "heck is a case conversion library")
    (description "This package provides heck is a case conversion library.")
    (license (list license:expat license:asl2.0))))

(define-public rust-dsl-auto-type-0.1
  (package
    (name "rust-dsl-auto-type")
    (version "0.1.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "dsl_auto_type" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "01xng43pn2dlc5k422is20dapq14w9x1p46qq968c0s167kapnf5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-darling" ,rust-darling-0.20)
                       ("rust-either" ,rust-either-1)
                       ("rust-heck" ,rust-heck-0.5)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://diesel.rs")
    (synopsis
     "Automatically expand query fragment types for factoring as functions")
    (description
     "This package provides Automatically expand query fragment types for factoring as functions.")
    (license (list license:expat license:asl2.0))))

(define-public rust-diesel-table-macro-syntax-0.2
  (package
    (name "rust-diesel-table-macro-syntax")
    (version "0.2.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "diesel_table_macro_syntax" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "09gvkyljhchbxfkxlkkrdcqcmcxwsim9sfljqilbq4x485b77710"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-syn" ,rust-syn-2))))
    (home-page "https://diesel.rs")
    (synopsis "Internal diesel crate")
    (description "This package provides Internal diesel crate.")
    (license (list license:expat license:asl2.0))))

(define-public rust-diesel-derives-2
  (package
    (name "rust-diesel-derives")
    (version "2.2.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "diesel_derives" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1n0qay7gaflj615k66rinmbswcwq133zax7r5s5qab1iwzhjpzyn"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-diesel-table-macro-syntax" ,rust-diesel-table-macro-syntax-0.2)
                       ("rust-dsl-auto-type" ,rust-dsl-auto-type-0.1)
                       ("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://diesel.rs")
    (synopsis
     "You should not use this crate directly, it is internal to Diesel")
    (description
     "This package provides You should not use this crate directly, it is internal to Diesel.")
    (license (list license:expat license:asl2.0))))

(define-public rust-diesel-2
  (package
    (name "rust-diesel")
    (version "2.2.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "diesel" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "151zn0n53mcri7fb7bwqrgir03mpaixc3a9g82iqyw5vc5rfx5xz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bigdecimal" ,rust-bigdecimal-0.1)
                       ("rust-bitflags" ,rust-bitflags-2)
                       ("rust-byteorder" ,rust-byteorder-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-diesel-derives" ,rust-diesel-derives-2)
                       ("rust-ipnet" ,rust-ipnet-2)
                       ("rust-ipnetwork" ,rust-ipnetwork-0.17)
                       ("rust-itoa" ,rust-itoa-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-libsqlite3-sys" ,rust-libsqlite3-sys-0.20)
                       ("rust-mysqlclient-src" ,rust-mysqlclient-src-0.1)
                       ("rust-mysqlclient-sys" ,rust-mysqlclient-sys-0.2)
                       ("rust-num-bigint" ,rust-num-bigint-0.2)
                       ("rust-num-integer" ,rust-num-integer-0.1)
                       ("rust-num-traits" ,rust-num-traits-0.2)
                       ("rust-percent-encoding" ,rust-percent-encoding-2)
                       ("rust-pq-src" ,rust-pq-src-0.3)
                       ("rust-pq-sys" ,rust-pq-sys-0.4)
                       ("rust-quickcheck" ,rust-quickcheck-1)
                       ("rust-r2d2" ,rust-r2d2-0.8)
                       ("rust-serde-json" ,rust-serde-json-0.9)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-url" ,rust-url-2)
                       ("rust-uuid" ,rust-uuid-0.7))))
    (home-page "https://diesel.rs")
    (synopsis
     "safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL")
    (description
     "This package provides a safe, extensible ORM and Query Builder for
@code{PostgreSQL}, SQLite, and @code{MySQL}.")
    (license (list license:expat license:asl2.0))))

(define-public rust-rust-decimal-1
  (package
    (name "rust-rust-decimal")
    (version "1.35.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "rust_decimal" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ymlnlykg2k9cxxs6wcy41sz6xij6cbazq70k49j30faq32d340p"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-arbitrary" ,rust-arbitrary-1)
                       ("rust-arrayvec" ,rust-arrayvec-0.7)
                       ("rust-borsh" ,rust-borsh-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-diesel" ,rust-diesel-1)
                       ("rust-diesel" ,rust-diesel-2)
                       ("rust-ndarray" ,rust-ndarray-0.15)
                       ("rust-num-traits" ,rust-num-traits-0.2)
                       ("rust-postgres-types" ,rust-postgres-types-0.2)
                       ("rust-proptest" ,rust-proptest-1)
                       ("rust-rand" ,rust-rand-0.8)
                       ("rust-rkyv" ,rust-rkyv-0.7)
                       ("rust-rocket" ,rust-rocket-0.5)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-tokio-postgres" ,rust-tokio-postgres-0.7))))
    (home-page "https://github.com/paupino/rust-decimal")
    (synopsis
     "Decimal number implementation written in pure Rust suitable for financial and fixed-precision calculations")
    (description
     "This package provides Decimal number implementation written in pure Rust suitable for financial and
fixed-precision calculations.")
    (license license:expat)))

(define-public rust-futures-intrusive-0.5
  (package
    (name "rust-futures-intrusive")
    (version "0.5.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "futures-intrusive" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0vwm08d1pli6bdaj0i7xhk3476qlx4pll6i0w03gzdnh7lh0r4qx"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-lock-api" ,rust-lock-api-0.4)
                       ("rust-parking-lot" ,rust-parking-lot-0.12))))
    (home-page "https://github.com/Matthias247/futures-intrusive")
    (synopsis
     "Futures based on intrusive data structures - for std and no-std environments.")
    (description
     "This package provides Futures based on intrusive data structures - for std and no-std environments.")
    (license (list license:expat license:asl2.0))))

(define-public rust-sqlx-core-0.7
  (package
    (name "rust-sqlx-core")
    (version "0.7.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sqlx-core" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1xiyr35dq10sf7lq00291svcj9wbaaz1ihandjmrng9a6jlmkfi4"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-ahash" ,rust-ahash-0.8)
                       ("rust-async-io" ,rust-async-io-1)
                       ("rust-async-std" ,rust-async-std-1)
                       ("rust-atoi" ,rust-atoi-2)
                       ("rust-bigdecimal" ,rust-bigdecimal-0.3)
                       ("rust-bit-vec" ,rust-bit-vec-0.6)
                       ("rust-bstr" ,rust-bstr-1)
                       ("rust-byteorder" ,rust-byteorder-1)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-chrono" ,rust-chrono-0.4)
                       ("rust-crc" ,rust-crc-3)
                       ("rust-crossbeam-queue" ,rust-crossbeam-queue-0.3)
                       ("rust-digest" ,rust-digest-0.10)
                       ("rust-either" ,rust-either-1)
                       ("rust-encoding-rs" ,rust-encoding-rs-0.8)
                       ("rust-event-listener" ,rust-event-listener-2)
                       ("rust-futures-channel" ,rust-futures-channel-0.3)
                       ("rust-futures-core" ,rust-futures-core-0.3)
                       ("rust-futures-intrusive" ,rust-futures-intrusive-0.5)
                       ("rust-futures-io" ,rust-futures-io-0.3)
                       ("rust-futures-util" ,rust-futures-util-0.3)
                       ("rust-hashlink" ,rust-hashlink-0.8)
                       ("rust-hex" ,rust-hex-0.4)
                       ("rust-indexmap" ,rust-indexmap-2)
                       ("rust-ipnetwork" ,rust-ipnetwork-0.20)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-mac-address" ,rust-mac-address-1)
                       ("rust-memchr" ,rust-memchr-2)
                       ("rust-native-tls" ,rust-native-tls-0.2)
                       ("rust-num-bigint" ,rust-num-bigint-0.4)
                       ("rust-once-cell" ,rust-once-cell-1)
                       ("rust-paste" ,rust-paste-1)
                       ("rust-percent-encoding" ,rust-percent-encoding-2)
                       ("rust-regex" ,rust-regex-1)
                       ("rust-rust-decimal" ,rust-rust-decimal-1)
                       ("rust-rustls" ,rust-rustls-0.21)
                       ("rust-rustls-pemfile" ,rust-rustls-pemfile-1)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-serde-json" ,rust-serde-json-1)
                       ("rust-sha1" ,rust-sha1-0.10)
                       ("rust-sha2" ,rust-sha2-0.10)
                       ("rust-smallvec" ,rust-smallvec-1)
                       ("rust-sqlformat" ,rust-sqlformat-0.2)
                       ("rust-thiserror" ,rust-thiserror-1)
                       ("rust-time" ,rust-time-0.3)
                       ("rust-tokio" ,rust-tokio-1)
                       ("rust-tokio-stream" ,rust-tokio-stream-0.1)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-url" ,rust-url-2)
                       ("rust-uuid" ,rust-uuid-1)
                       ("rust-webpki-roots" ,rust-webpki-roots-0.25))))
    (home-page "https://github.com/launchbadge/sqlx")
    (synopsis
     "Core of SQLx, the rust SQL toolkit. Not intended to be used directly")
    (description
     "This package provides Core of SQLx, the rust SQL toolkit.  Not intended to be used directly.")
    (license (list license:expat license:asl2.0))))

(define-public rust-sqlx-0.7
  (package
    (name "rust-sqlx")
    (version "0.7.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "sqlx" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1ahadprvyhjraq0c5712x3kdkp1gkwfm9nikrmcml2h03bzwr8n9"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-sqlx-core" ,rust-sqlx-core-0.7)
                       ("rust-sqlx-macros" ,rust-sqlx-macros-0.7)
                       ("rust-sqlx-mysql" ,rust-sqlx-mysql-0.7)
                       ("rust-sqlx-postgres" ,rust-sqlx-postgres-0.7)
                       ("rust-sqlx-sqlite" ,rust-sqlx-sqlite-0.7))
       #:cargo-development-inputs (("rust-anyhow" ,rust-anyhow-1)
                                   ("rust-async-std" ,rust-async-std-1)
                                   ("rust-criterion" ,rust-criterion-0.5)
                                   ("rust-dotenvy" ,rust-dotenvy-0.15)
                                   ("rust-env-logger" ,rust-env-logger-0.11)
                                   ("rust-futures" ,rust-futures-0.3)
                                   ("rust-hex" ,rust-hex-0.4)
                                   ("rust-libsqlite3-sys" ,rust-libsqlite3-sys-0.27)
                                   ("rust-paste" ,rust-paste-1)
                                   ("rust-rand" ,rust-rand-0.8)
                                   ("rust-rand-xoshiro" ,rust-rand-xoshiro-0.6)
                                   ("rust-serde" ,rust-serde-1)
                                   ("rust-serde-json" ,rust-serde-json-1)
                                   ("rust-tempfile" ,rust-tempfile-3)
                                   ("rust-time" ,rust-time-0.3)
                                   ("rust-tokio" ,rust-tokio-1)
                                   ("rust-trybuild" ,rust-trybuild-1)
                                   ("rust-url" ,rust-url-2))))
    (home-page "https://github.com/launchbadge/sqlx")
    (synopsis
     "ð§° The Rust SQL Toolkit. An async, pure Rust SQL crate featuring compile-time checked queries without a DSL. Supports PostgreSQL, MySQL, and SQLite")
    (description
     "This package provides ð§° The Rust SQL Toolkit.  An async, pure Rust SQL crate featuring compile-time
checked queries without a DSL. Supports @code{PostgreSQL}, @code{MySQL}, and
SQLite.")
    (license (list license:expat license:asl2.0))))

(define-public rust-clippy-0.0.302
  (package
    (name "rust-clippy")
    (version "0.0.302")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "clippy" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1562x3sq9mgmc8j39gd34wqm7ybrdvpmj7cc1n450gwsawayw4fr"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-term" ,rust-term-0.5))))
    (home-page "https://github.com/rust-lang-nursery/rust-clippy")
    (synopsis "bunch of helpful lints to avoid common pitfalls in Rust.")
    (description
     "This package provides a bunch of helpful lints to avoid common pitfalls in Rust.")
    (license (list license:expat license:asl2.0))))

(define-public rust-libc-0.2
  (package
    (name "rust-libc")
    (version "0.2.155")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "libc" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0z44c53z54znna8n322k5iwg80arxxpdzjj5260pxxzc9a58icwp"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-rustc-std-workspace-core" ,rust-rustc-std-workspace-core-1))))
    (home-page "https://github.com/rust-lang/libc")
    (synopsis "Raw FFI bindings to platform libraries like libc.")
    (description
     "This package provides Raw FFI bindings to platform libraries like libc.")
    (license (list license:expat license:asl2.0))))

(define-public rust-cfg-aliases-0.2
  (package
    (name "rust-cfg-aliases")
    (version "0.2.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "cfg_aliases" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "092pxdc1dbgjb6qvh83gk56rkic2n2ybm4yvy76cgynmzi3zwfk1"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t))
    (home-page "https://github.com/katharostech/cfg_aliases")
    (synopsis
     "tiny utility to help save you a lot of effort with long winded `#[cfg()]` checks.")
    (description
     "This package provides a tiny utility to help save you a lot of effort with long
winded `#[cfg()]` checks.")
    (license license:expat)))

(define-public rust-nix-0.29
  (package
    (name "rust-nix")
    (version "0.29.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "nix" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0ikvn7s9r2lrfdm3mx1h7nbfjvcc6s9vxdzw7j5xfkd2qdnp9qki"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-bitflags" ,rust-bitflags-2)
                       ("rust-cfg-if" ,rust-cfg-if-1)
                       ("rust-cfg-aliases" ,rust-cfg-aliases-0.2)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-memoffset" ,rust-memoffset-0.9)
                       ("rust-pin-utils" ,rust-pin-utils-0.1))))
    (home-page "https://github.com/nix-rust/nix")
    (synopsis "Rust friendly bindings to *nix APIs")
    (description "This package provides Rust friendly bindings to *nix APIs.")
    (license license:expat)))

(define-public rust-mio-0.8
  (package
    (name "rust-mio")
    (version "0.8.11")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mio" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "034byyl0ardml5yliy1hmvx8arkmn9rv479pid794sm07ia519m4"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-libc" ,rust-libc-0.2)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-wasi" ,rust-wasi-0.11)
                       ("rust-windows-sys" ,rust-windows-sys-0.48))))
    (home-page "https://github.com/tokio-rs/mio")
    (synopsis "Lightweight non-blocking I/O")
    (description "This package provides Lightweight non-blocking I/O.")
    (license license:expat)))

(define-public rust-mio-aio-0.9
  (package
    (name "rust-mio-aio")
    (version "0.9.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mio-aio" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1zrp159m5nc1j23awbrn9rmx7c5lh47z9szg65s3zj4h95b0v5wl"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-mio" ,rust-mio-0.8)
                       ("rust-nix" ,rust-nix-0.29)
                       ("rust-pin-utils" ,rust-pin-utils-0.1))))
    (home-page "https://github.com/asomers/mio-aio")
    (synopsis "POSIX AIO bindings for mio")
    (description "This package provides POSIX AIO bindings for mio.")
    (license (list license:expat license:asl2.0))))

(define-public rust-tokio-macros-2
  (package
    (name "rust-tokio-macros")
    (version "2.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tokio-macros" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "0lnpg14h1v3fh2jvnc8cz7cjf0m7z1xgkwfpcyy632g829imjgb9"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-proc-macro2" ,rust-proc-macro2-1)
                       ("rust-quote" ,rust-quote-1)
                       ("rust-syn" ,rust-syn-2))))
    (home-page "https://tokio.rs")
    (synopsis "Tokio's proc macros.")
    (description "This package provides Tokio's proc macros.")
    (license license:expat)))

(define-public rust-hermit-abi-0.3
  (package
    (name "rust-hermit-abi")
    (version "0.3.9")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "hermit-abi" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "092hxjbjnq5fmz66grd9plxd0sh6ssg5fhgwwwqbrzgzkjwdycfj"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-compiler-builtins" ,rust-compiler-builtins-0.1)
                       ("rust-rustc-std-workspace-alloc" ,rust-rustc-std-workspace-alloc-1)
                       ("rust-rustc-std-workspace-core" ,rust-rustc-std-workspace-core-1))))
    (home-page "https://github.com/hermit-os/hermit-rs")
    (synopsis "Hermit system calls definitions")
    (description "This package provides Hermit system calls definitions.")
    (license (list license:expat license:asl2.0))))

(define-public rust-mio-1
  (package
    (name "rust-mio")
    (version "1.0.1")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "mio" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1r5g65s5acsx440m0a3pylclbrd0dqz93hg15k9crpllsdbf8sa5"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build? #t
       #:cargo-inputs (("rust-hermit-abi" ,rust-hermit-abi-0.3)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-wasi" ,rust-wasi-0.11)
                       ("rust-windows-sys" ,rust-windows-sys-0.52))))
    (home-page "https://github.com/tokio-rs/mio")
    (synopsis "Lightweight non-blocking I/O")
    (description "This package provides Lightweight non-blocking I/O.")
    (license license:expat)))

(define-public rust-tokio-1
  (package
    (name "rust-tokio")
    (version "1.39.2")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "tokio" version))
       (file-name (string-append name "-" version ".tar.gz"))
       (sha256
        (base32 "1cb7yhba7nnf00cylcywk7rai5kkdb8b4jzwrc26zgbqqwdzp96s"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs (("rust-backtrace" ,rust-backtrace-0.3)
                       ("rust-bytes" ,rust-bytes-1)
                       ("rust-libc" ,rust-libc-0.2)
                       ("rust-mio" ,rust-mio-1)
                       ("rust-parking-lot" ,rust-parking-lot-0.12)
                       ("rust-pin-project-lite" ,rust-pin-project-lite-0.2)
                       ("rust-signal-hook-registry" ,rust-signal-hook-registry-1)
                       ("rust-socket2" ,rust-socket2-0.5)
                       ("rust-tokio-macros" ,rust-tokio-macros-2)
                       ("rust-tracing" ,rust-tracing-0.1)
                       ("rust-windows-sys" ,rust-windows-sys-0.52))
       #:cargo-development-inputs (("rust-async-stream" ,rust-async-stream-0.3)
                                   ("rust-futures" ,rust-futures-0.3)
                                   ("rust-libc" ,rust-libc-0.2)
                                   ("rust-loom" ,rust-loom-0.7)
                                   ("rust-mio-aio" ,rust-mio-aio-0.9)
                                   ("rust-mockall" ,rust-mockall-0.11)
                                   ("rust-nix" ,rust-nix-0.29)
                                   ("rust-rand" ,rust-rand-0.8)
                                   ("rust-socket2" ,rust-socket2-0.5)
                                   ("rust-tempfile" ,rust-tempfile-3)
                                   ("rust-tokio-stream" ,rust-tokio-stream-0.1)
                                   ("rust-tokio-test" ,rust-tokio-test-0.4)
                                   ("rust-wasm-bindgen-test" ,rust-wasm-bindgen-test-0.3)
                                   ("rust-windows-sys" ,rust-windows-sys-0.52))))
    (home-page "https://tokio.rs")
    (synopsis
     "An event-driven, non-blocking I/O platform for writing asynchronous I/O
backed applications.")
    (description
     "This package provides An event-driven, non-blocking I/O platform for writing asynchronous I/O backed
applications.")
    (license license:expat)))

