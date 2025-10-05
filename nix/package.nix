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

  nativeBuildInputs = with pkgsBuildHost; [ cmake ];
  buildInputs = with pkgsHostTarget; [
    libopus
  ];
  propagatedBuildInputs = with pkgsHostTarget; [
    yt-dlp-light
    (lib.getBin ffmpeg-headless)
  ];
}
