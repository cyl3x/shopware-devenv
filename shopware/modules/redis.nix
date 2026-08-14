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
    services.redis.extraConfig = "locale-collate C";

    languages.php.ini = ''
      session.save_handler = redis
      session.save_path = "tcp://127.0.0.1:${toString config.services.redis.port}/0"
    '';
  };
}
