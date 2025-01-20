{
  description = "Wrapper for the Shopware devenv development environment";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-flake.url = "github:juspay/rust-flake";
    rust-flake.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.flake-parts.flakeModules.easyOverlay
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];

      systems = [ "aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux" ];

      perSystem = { config, self', inputs', pkgs, system, ... }: {
        rust-project = {
          crates.swde.crane.args = {};
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (builtins.match ".*/src/.*\.nix$" path != null) ||
              (config.rust-project.crane-lib.filterCargoSources path type)
            ;
          };
        };

        overlayAttrs = { inherit (self'.packages) swde; };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self'.devShells.rust ];

          RUST_LOG = "info";
          RUST_BACKTRACE = "full";
        };
      };
    };
}
