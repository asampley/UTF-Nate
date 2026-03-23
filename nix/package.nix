{
  lib,
  pkgsHostTarget,
  pkgsBuildHost,
  ...
}:
let
  manifest = (lib.importTOML ../Cargo.toml);
in

pkgsHostTarget.rustPlatform.buildRustPackage {
  meta.mainProgram = "utf-nate";

  pname = manifest.package.name;
  version = manifest.package.version;
  cargoLock.lockFile = ../Cargo.lock;
  cargoLock.outputHashes = {
    "songbird-0.5.0" = "sha256-YleLMN7Mnta4etqKRXZpWSPgc1PblFAWwgUflGmKYsI=";
  };
  src =
    with lib.fileset;
    toSource {
      root = ../.;
      fileset = unions [
        ../Cargo.toml
        ../Cargo.lock
        ../src
        ../templates
        ../resources
      ];
    };
  nativeBuildInputs = with pkgsBuildHost; [
    coreutils
    makeWrapper
    pkg-config
  ];
  buildInputs = with pkgsHostTarget; [
    libopus
  ];
  postInstall = ''
    wrapProgram $out/bin/utf-nate --prefix PATH : ${with pkgsHostTarget; lib.makeBinPath [
      yt-dlp-light
      ffmpeg-headless
    ]}

    cp -r resources $out/resources
  '';
}
