{
  description = "Wrapper for the Shopware devenv development environment";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }: {

    packages.x86_64-linux.default = let
      pkgs = import nixpkgs {
        overlays = [ self.overlays.default ];
        system = "x86_64-linux";
      };
    in pkgs.swde;

    devShell.x86_64-linux = let
      pkgs = import nixpkgs {
        overlays =  [ (import rust-overlay) ];
        system = "x86_64-linux";
      };
    in with pkgs; mkShell {
      buildInputs = [ rust-bin.stable."1.75.0".default ];
    };
  };
}
