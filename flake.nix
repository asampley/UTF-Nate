{
  description = "UTF-Nate";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems = {
      url = nix/systems.nix;
      flake = false;
    };
  };
  outputs =
    inputs:
    let
      systems = import inputs.systems;
      lib = inputs.nixpkgs.lib;
      genSystems = lib.genAttrs systems;
      pkgsFor =
        localSystem: crossSystem:
        import inputs.nixpkgs {
          localSystem = {
            system = localSystem;
          };
          crossSystem = {
            system = crossSystem;
          };
        };
      package = import nix/package.nix;
      docker = import nix/docker.nix;
      postfix =
        localSystem: crossSystem: if localSystem == crossSystem then "" else "-for-${crossSystem}";
      genSystemsCross =
        f:
        lib.genAttrs systems (
          system:
          lib.mergeAttrsList (
            map (
              cross:
              let
                pkgs = pkgsFor system cross;
                post = postfix system cross;
              in
              (f {
                inherit
                  pkgs
                  post
                  system
                  cross
                  ;
              })
            ) systems
          )
        );
      shell =
        { pkgsBuildHost, ... }:
        pkgsBuildHost.mkShell {
          inputsFrom = [ (pkgsBuildHost.callPackage package { }) ];
          nativeBuildInputs = with pkgsBuildHost; [
            cargo-audit
            clippy
            rust-analyzer
            rustfmt

            # For tls-native-tls, which is not the default, but is checked
            # during commit that it compiles
            pkg-config
            openssl
          ];
        };
    in
    {
      formatter = genSystems (system: (pkgsFor system system).nixfmt-rfc-style);

      bundlers = genSystemsCross (
        { pkgs, post, ... }:
        {
          "docker${post}" = drv: pkgs.callPackage docker { inherit drv; };
        }
      );

      packages = genSystemsCross (
        {
          pkgs,
          post,
          system,
          cross,
          ...
        }:
        let
          utf-nate = pkgs.callPackage package { };
        in
        {
          "utf-nate${post}" = utf-nate;
        }
        // (if system == cross then { "default" = utf-nate; } else { })
      );

      devShells = genSystems (
        system:
        let
          pkgs = (pkgsFor system system);
          utf-nate-shell = pkgs.callPackage shell { };
        in
        {
          "utf-nate" = utf-nate-shell;
          "default" = utf-nate-shell;
        }
      );
    };
}
