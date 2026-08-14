{ config, pkgs, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.javascript;
in with lib; {
  options.shopware.modules.javascript = {
    enable = mkOption {
      description = "Enable javascript and necessary configuration.";
      type = types.bool;
      default = true;
    };
    auto-version = mkOption {
      description = "javascript version auto-detected based on the shopware version.";
      readOnly = true;
      type = types.package;
      default = if config.shopware.version == "6.4" then pkgs.nodejs_20
        else if config.shopware.version == "6.5" then pkgs.nodejs_20
        else if config.shopware.version == "6.6" then pkgs.nodejs_22
        else if config.shopware.version == "6.7" then pkgs.nodejs_24
        else pkgs.nodejs_26;
      defaultText = "<nodejs_20 for 6.4 and 6.5, nodejs_22 for 6.6, nodejs_24 for 6.7, nodejs_26 otherwise>";
    };
  };

  config = mkIf cfg.enable {
    languages.javascript = {
      enable = true;
      package = mkDefault cfg.auto-version;
    };
  };
}
