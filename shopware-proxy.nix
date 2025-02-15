{ config, lib, ... }: with lib; let
  concatStrings = builtins.concatStringsSep ''
  '';

  cfg = config.shopware-proxy;

  genProxy = domain: port: ''
    https://${domain}:${toString cfg.port} {
      reverse_proxy http://127.0.0.1:${toString port}
    }
  '';

  genPlatform = domain: port: concatStrings [
    (optionalString cfg.modules.admin-watcher (genProxy domain port))
    (optionalString cfg.modules.admin-watcher (genProxy "admin.${domain}" (port + 1)))
    (optionalString cfg.modules.store-watcher (genProxy "store.${domain}" (port + 2)))
    (optionalString cfg.modules.adminer (genProxy "adminer.${domain}" (port + 3)))
    (optionalString cfg.modules.mailpit (genProxy "mailpit.${domain}" (port + 7)))
    (optionalString cfg.modules.rabbitmq (genProxy "rabbitmq.${domain}" (port + 11)))
  ];
in {
  options.shopware-proxy = {
    enable = mkOption {
      description = "Enable the proxy used for shopware and app servers.";
      type = types.bool;
      default = false;
    };
    port = mkOption {
      description = "Port on which the proxy listens.";
      type = types.port;
      default = 2000;
    };
    platforms = mkOption {
      description = "Map of platform projects to proxy.";
      example = ''
        {
          "trunk.localhost" = 3000; # base port of platform, defined by `shopware.port`
          "66.localhost" = 4000;
        }
      '';
      type = types.attrsOf types.port;
      default = {};
    };
    apps = mkOption {
      description = "Map of app servers to proxy.";
      type = types.attrsOf types.port;
      default = {};
    };
    modules = {
      admin-watcher = mkOption {
        description = "Enable reverse proxy for the admin-watcher.";
        type = types.bool;
        default = true;
      };
      store-watcher = mkOption {
        description = "Enable reverse proxy for the store-watcher.";
        type = types.bool;
        default = true;
      };
      adminer = mkOption {
        description = "Enable reverse proxy for adminer.";
        type = types.bool;
        default = true;
      };
      mailpit = mkOption {
        description = "Enable reverse proxy for mailpit.";
        type = types.bool;
        default = true;
      };
      rabbitmq = mkOption {
        description = "Enable reverse proxy for rabbitmq management.";
        type = types.bool;
        default = false;
      };
    };
  };

  config = mkIf cfg.enable {
    services.caddy.enable = true;
    services.caddy.config = ''
      {
        auto_https disable_redirects
        skip_install_trust
      }

      ${concatStrings (builtins.attrValues (builtins.mapAttrs genPlatform cfg.platforms))}

      ${concatStrings (builtins.attrValues (builtins.mapAttrs genProxy cfg.apps))}
    '';
  };
}
