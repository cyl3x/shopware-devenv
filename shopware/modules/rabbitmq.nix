{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.rabbitmq;
in with lib; {
  options.shopware.modules.rabbitmq = {
    enable = mkOption {
      description = "Enable rabbitmq and necessary configuration.";
      type = types.bool;
      default = false;
    };
    domain = mkOption {
      description = "Domain on which rabbitmq management is available.";
      readOnly = true;
      type = types.str;
      default = "rabbitmq.${config.shopware.domain}";
    };
    port = mkOption {
      description = "Port on which rabbitmq is available.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 10;
    };
    management-port = mkOption {
      description = "Port on which rabbitmq management is available.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 11;
    };
  };

  config = mkMergeIf cfg.enable [
    (mkCaddyProxy {
      inherit (cfg) domain;
      port = cfg.management-port;
    })

    {
      env.MESSENGER_TRANSPORT_DSN = mkDefault "amqp://guest:guest@localhost:${toString cfg.port}/%2f/messages";
      env.MESSENGER_TRANSPORT_LOW_PRIORITY_DSN = mkDefault "amqp://guest:guest@localhost:${toString cfg.port}/%2f/messages";
      env.MESSENGER_TRANSPORT_FAILURE_DSN = mkDefault "amqp://guest:guest@localhost:${toString cfg.port}/%2f/messages";

      services.rabbitmq = {
        enable = mkDefault true;
        port = mkDefault cfg.port;
        managementPlugin.enable = mkDefault true;
        managementPlugin.port = mkDefault cfg.management-port;
        nodeName = mkDefault "rabbitmq@${config.shopware.domain}";
      };
    }
  ];
}
