{ self, pkgs, lib, ... }:
pkgs.rustPlatform.buildRustPackage {
  pname = "swde";
  inherit ((lib.importTOML (self + "/Cargo.toml")).package) version;

  src = self;

  cargoLock.lockFile = self + "/Cargo.lock";
}