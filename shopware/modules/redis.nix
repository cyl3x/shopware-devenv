{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.redis;
in with lib; {
  options.shopware.modules.redis = {
    enable = mkOption { type = types.bool; default = true; };
    port = mkOption {
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 5;
    };
  };

  config = mkIf cfg.enable {
    services.redis.enable = mkDefault true;
    services.redis.port = mkDefault cfg.port;
  };
}
