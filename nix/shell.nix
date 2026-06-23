{
  mkShell,
  cargo-audit,
  clippy,
  rust-analyzer,
  rustfmt,
  pkg-config,
  openssl,
  utf-nate,
  yt-dlp-light,
  ffmpeg-headless,
  ...
}:
mkShell {
  inputsFrom = [ utf-nate ];
  nativeBuildInputs = [
    cargo-audit
    clippy
    rust-analyzer
    rustfmt

    # For tls-native-tls, which is not the default, but is checked
    # during commit that it compiles
    pkg-config
    openssl

    # Path for runtime
    yt-dlp-light
    ffmpeg-headless
  ];
}
