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
