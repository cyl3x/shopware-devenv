{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.mailpit;
in with lib; {
  options.shopware.modules.mailpit = {
    enable = mkOption {
      description = "Enable mailpit and necessary configuration.";
      type = types.bool;
      default = true;
    };
    domain = mkOption {
      description = "Domain on which mailpit is available.";
      readOnly = true;
      type = types.str;
      default = "mailpit.${config.shopware.domain}";
    };
    port = mkOption {
      description = "Port on which mailpit is available.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 7;
    };
    smtp-port = mkOption {
      description = "Port on which mailpit listens for SMTP connections.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 8;
    };
  };

  config = mkMergeIf cfg.enable [
    (mkCaddyProxy {
      inherit (cfg) domain port;
    })

    {
      env.MAILER_DSN = mkDefault "smtp://127.0.0.1:${toString cfg.smtp-port}";

      services.mailpit.enable = mkDefault true;
      services.mailpit.smtpListenAddress = mkDefault "127.0.0.1:${toString cfg.smtp-port}";
      services.mailpit.uiListenAddress = mkDefault "127.0.0.1:${toString cfg.port}";
    }
  ];
}
