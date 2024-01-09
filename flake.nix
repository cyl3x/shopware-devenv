{
  description = "Wrapper for the Shopware devenv development environment";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs, ... }: {
    overlays.default = final: prev: { swde = prev.pkgs.callPackage ./. { inherit self; }; };

    packages.x86_64-linux.default = let
      pkgs = import nixpkgs {
        overlays = [ self.overlays.default ];
        system = "x86_64-linux";
      };
    in pkgs.swde;
  };
}
