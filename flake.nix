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
      lib = nixpkgs.lib;
      forAllSystems = lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
      manifest = (lib.importTOML ./Cargo.toml).package;
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
      docker = drv: {
        name = "utf-nate";
        tag = manifest.version;
        contents = [ drv ] ++ drv.propagatedBuildInputs;
        config.Entrypoint = [ "/bin/utf-nate" ];
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
          utf-nate-docker = pkgsFor.${system}.dockerTools.buildLayeredImage (docker utf-nate);

          default = utf-nate;
        } // builtins.listToAttrs (builtins.concatMap
          (cross:
            let
              cross-nixpkgs = (import nixpkgs { inherit system; crossSystem = { config = cross; }; });
              cross-package = cross-nixpkgs.callPackage package { };
            in [
              {
                name = "utf-nate-${cross}";
                value = cross-package;
              }
              {
                name = "utf-nate-docker-${cross}";
                value = cross-nixpkgs.dockerTools.buildLayeredImage (docker cross-package);
              }
            ]
          )
          crossSystems
        )
      );

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage shell { };
      });
    };
}
