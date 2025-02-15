{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.admin-watcher;
in with lib; {
  options.shopware.modules.admin-watcher = {
    enable = mkOption {
      description = "Enable configuration for the admin-watcher.";
      type = types.bool;
      default = true;
    };
    domain = mkOption {
      description = "Domain on which the admin-watcher is available.";
      readOnly = true;
      type = types.str;
      default = "admin.${config.shopware.domain}";
    };
    port = mkOption {
      description = "Port on which the admin-watcher is available.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 1;
    };
  };

  config = mkMergeIf cfg.enable [
    (mkCaddyProxy {
      inherit (cfg) domain port;
      http = false;
      host = "localhost";
    }) 

    {
      env.HOST = mkDefault cfg.domain;
      env.PORT = mkDefault (toString cfg.port);
      env.ADMIN_PORT = mkDefault (toString cfg.port);
    }
  ];
}
