{ pkgs, ... }:

{
    languages.rust = {
        enable = true;
        channel = "nixpkgs";
        components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
    };
}
