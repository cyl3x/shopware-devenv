{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.redis;
in with lib; {
  options.shopware.modules.redis = {
    enable = mkOption {
      description = "Enable redis and necessary configuration.";
      type = types.bool;
      default = true;
    };
    port = mkOption {
      description = "Port on which the redis is available.";
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
