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
      description = "Port on which elasticsearch is available. Allocated by devenv, starting from `shopware.port + 12`.";
      readOnly = true;
      type = types.port;
      default = config.processes.opensearch.ports.http.value;
      defaultText = "<allocated, starting from shopware.port + 12>";
    };
    tcp-port = mkOption {
      description = "TCP port on which elasticsearch is available. Allocated by devenv, starting from `shopware.port + 13`.";
      readOnly = true;
      type = types.port;
      default = config.processes.opensearch.ports.transport.value;
      defaultText = "<allocated, starting from shopware.port + 13>";
    };
  };

  config = mkIf cfg.enable {
    env.OPENSEARCH_URL = mkDefault "http://127.0.0.1:${toString cfg.port}";
    env.ADMIN_OPENSEARCH_URL = mkDefault "http://127.0.0.1:${toString cfg.port}";
    env.SHOPWARE_ES_THROW_EXCEPTION = mkDefault "1";

    services.opensearch = {
      enable = mkDefault true;
      settings."http.port" = mkDefault (config.shopware.port + 12);
      settings."transport.port" = mkDefault (config.shopware.port + 13);
    };
  };
}
