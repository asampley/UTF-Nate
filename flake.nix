{
  description = "UTF-Nate";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs =
    { nixpkgs, ... }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      crossSystems = [
        "aarch64-unknown-linux-gnu"
        "x86_64-unknown-linux-gnu"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
      manifest = (nixpkgs.lib.importTOML ./Cargo.toml).package;
      package = { pkgsHostTarget, pkgsBuildHost, ... }:
        pkgsHostTarget.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgsBuildHost.lib.cleanSource ./.;

          nativeBuildInputs = with pkgsBuildHost; [ cmake ];
          buildInputs = with pkgsHostTarget; [
            libopus
          ];
          propagatedBuildInputs = with pkgsHostTarget; [
            yt-dlp
            ffmpeg-headless
          ];
        };
      shell = { pkgsBuildHost, ... }:
        pkgsBuildHost.mkShell {
          inputsFrom = [ (pkgsBuildHost.callPackage package { }) ];
          buildInputs = with pkgsBuildHost; [
            rust-analyzer
            rustfmt
            clippy
          ];
        };
    in
    {
      formatter = forAllSystems (system: pkgsFor.${system}.nixfmt-rfc-style);
      packages = forAllSystems (system:
        rec {
          utf-nate = pkgsFor.${system}.callPackage package { };

          default = utf-nate;
        } // builtins.listToAttrs (map
          (cross: {
            name = "utf-nate-${cross}";
            value = (import nixpkgs { inherit system; crossSystem = { config = cross; }; }).callPackage package { };
          })
          crossSystems
        )
      );

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage shell { };
      });
    };
}
