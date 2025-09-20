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
  pname = manifest.package.name;
  version = manifest.package.version;
  cargoLock.lockFile = ../Cargo.lock;
  src = pkgsBuildHost.lib.cleanSource ../.;

  nativeBuildInputs = with pkgsBuildHost; [ cmake ];
  buildInputs = with pkgsHostTarget; [
    libopus
  ];
  propagatedBuildInputs = with pkgsHostTarget; [
    yt-dlp-light
    (lib.getBin ffmpeg-headless)
  ];
}
