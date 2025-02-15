{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.elasticsearch;
in with lib; {
  options.shopware.modules.elasticsearch = {
    enable = mkOption {
      description = "Enable elasticsearch and necessary configuration.";
      type = types.bool;
      default = false;
    };
    port = mkOption {
      description = "Port on which elasticsearch is available.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 12;
    };
    tcp-port = mkOption {
      description = "TCP port on which elasticsearch is available.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 13;
    };
  };

  config = mkIf cfg.enable {
    env.OPENSEARCH_URL = mkDefault "http://127.0.0.1:${toString cfg.port}";
    env.ADMIN_OPENSEARCH_URL = mkDefault "http://127.0.0.1:${toString cfg.port}";
    env.SHOPWARE_ES_THROW_EXCEPTION = mkDefault "1";

    services.opensearch = {
      enable = mkDefault true;
      settings."http.port" = mkDefault cfg.port;
      settings."transport.port" = mkDefault cfg.tcp-port;
    };
  };
}
