{ self, pkgs, lib, ... }: let
  cargo = (lib.importTOML ./Cargo.toml);
in pkgs.rustPlatform.buildRustPackage {
  pname = cargo.package.name;
  version = cargo.package.version;

  src = self;
  cargoLock.lockFile = ./Cargo.lock; 
  nativeBuildInputs = [
    pkgs.rust-bin.stable."1.75.0".default
  ];

  meta = with lib; {
    description = cargo.package.description;
    homepage = cargo.package.repository;
    license = with licenses; [ mit ];
    maintainers = with maintainers; cargo.package.authors;
  };
}