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
    enable = mkOption { type = types.bool; default = false; };
    port = mkOption { type = types.port; default = 2000; };
    platforms = mkOption {
      type = types.attrsOf types.port;
      default = {};
    };
    apps = mkOption {
      type = types.attrsOf types.port;
      default = {};
    };
    modules = {
      admin-watcher = mkOption { type = types.bool; default = true; };
      store-watcher = mkOption { type = types.bool; default = true; };
      adminer = mkOption { type = types.bool; default = true; };
      mailpit = mkOption { type = types.bool; default = true; };
      rabbitmq = mkOption { type = types.bool; default = false; };
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
