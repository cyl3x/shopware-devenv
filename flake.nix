{
  description = "Wrapper for the Shopware devenv development environment";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" ];
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      commonArgs = let
        toml = (pkgs.lib.importTOML ./Cargo.toml);
      in {
        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = path: type:
            (builtins.match ".*/src/.*\.nix$" path != null) || (craneLib.filterCargoSources path type);
        };
        
        meta = with pkgs.lib; {
          description = toml.package.description;
          homepage = toml.package.repository;
          license = [ licenses.mit ];
          maintainers = toml.package.authors;
        };
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      swde = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
    in {
      checks = { inherit swde; };
      packages = {
        inherit swde;
        default = swde;
      };

      devShells.default = craneLib.devShell {
        inputsFrom = [ swde ];
        packages = with pkgs; [ rust-analyzer rustToolchain ];
        RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
      };
    }) // {
      overlays = rec {
        swde = final: prev: { swde = self.packages.${prev.system}.swde; };
        default = swde;
      };
    };
}
