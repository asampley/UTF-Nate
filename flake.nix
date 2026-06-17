{
  description = "UTF-Nate";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems = {
      url = nix/systems.nix;
      flake = false;
    };
    nix-pkgset = {
      url = "github:szlend/nix-pkgset";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{
      self,
      nixpkgs,
      nix-pkgset,
      ...
    }:
    let
      systems = import inputs.systems;
      genSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      formatter = genSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-tree);

      bundlers = genSystems (
        system:
        let
          pkgs = (self.lib.makePackageSet nixpkgs.legacyPackages.${system});
        in
        pkgs.bundlers
        // {
          cross = builtins.mapAttrs (name: value: value.bundlers) pkgs.cross;
        }
      );

      legacyPackages = genSystems (system: self.lib.makePackageSet nixpkgs.legacyPackages.${system});

      packages = genSystems (
        system:
        nixpkgs.lib.filterAttrs (_: nixpkgs.lib.isDerivation) self.legacyPackages.${system}
        // {
          default = self.packages.${system}.utf-nate;
        }
      );

      devShells = genSystems (system: {
        default = self.legacyPackages.${system}.utf-nate-pkgs.callPackage nix/shell.nix { };
      });

      lib = {
        makePackageSet =
          pkgs:
          nix-pkgset.lib.makePackageSet "utf-nate-pkgs" pkgs.newScope (utf-nate-pkgs: {
            utf-nate = utf-nate-pkgs.callPackage nix/package.nix { };
            bundlers = {
              docker = drv: utf-nate-pkgs.callPackage nix/docker.nix { inherit drv; };
            };
            cross = genSystems (
              system: self.lib.makePackageSet (import nixpkgs { localSystem = pkgs.stdenv.buildPlatform; crossSystem = system; })
            );
          });
      };
    };
}
