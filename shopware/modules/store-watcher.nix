
{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.store-watcher;
in with lib; {
  options.shopware.modules.store-watcher = {
    enable = mkOption { type = types.bool; default = true; };
    domain = mkOption {
      readOnly = true;
      type = types.str;
      default = "store.${config.shopware.domain}";
    };
    port = mkOption {
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 2;
    };
    asset-port = mkOption {
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
