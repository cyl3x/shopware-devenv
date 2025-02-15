
{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.store-watcher;
in with lib; {
  options.shopware.modules.store-watcher = {
    enable = mkOption {
      description = "Enable configuration for the store-watcher";
      type = types.bool;
      default = true;
    };
    domain = mkOption {
      description = "Domain on which the store-watcher is available.";
      readOnly = true;
      type = types.str;
      default = "store.${config.shopware.domain}";
    };
    port = mkOption {
      description = "Port on which the store-watcher is available.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 2;
    };
    asset-port = mkOption {
      description = "Port on which the store-watcher serves assets.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 4;
    };
  };

  config = mkMergeIf cfg.enable [
    (mkCaddyProxy {
      inherit (cfg) domain port;
      http = false;
    }) 

    {
      env.PROXY_URL = mkDefault "http://127.0.0.1:${toString cfg.port}";
      env.STOREFRONT_PROXY_PORT = mkDefault (toString cfg.port);
      env.STOREFRONT_ASSETS_PORT = mkDefault (toString cfg.asset-port);
      env.STOREFRONT_SKIP_SSL_CERT = mkDefault true;
    }
  ];
}
