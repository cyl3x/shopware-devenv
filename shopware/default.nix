{ config, ... }@args: let
  lib = import ./lib.nix args;

  cfg = config.shopware;
in with lib; {
  # ordering is important, as last config comes first
  # caddyfile needs to be ordered
  imports = [
    ./ssl.nix

    ./modules/admin-watcher.nix
    ./modules/adminer.nix
    ./modules/cypress.nix
    ./modules/elasticsearch.nix
    ./modules/mailpit.nix
    ./modules/mysql.nix
    ./modules/rabbitmq.nix
    ./modules/redis.nix
    ./modules/store-watcher.nix
    ./modules/var-dump-server.nix

    ./extras/start-proxy.nix
    ./extras/xdebug.nix

    ./caddy.nix
  ];

  options.shopware = {
    enable = mkOption { type = types.bool; default = false; };
    domain = mkOption {
      type = types.str;
      default = let
        basename = strings.removePrefix "platform_" (builtins.baseNameOf config.env.DEVENV_ROOT);
        subdomain = if basename == "platform" then "trunk" else basename;
      in subdomain + ".localhost";
    };
    port = mkOption { type = types.port; default = 3000; };

    protocol = mkOption {
      readOnly = true;
      type = types.enum ["http" "https"];
      default = if cfg.ssl.proxy.enable || cfg.ssl.standalone.enable then "https" else "http";
    };
  };

  config = mkIf cfg.enable {
    assertions = [
      {
        assertion = !(cfg.ssl.standalone.enable && cfg.ssl.proxy.enable);
        message = ''
          `shopware.ssl.standalone.enable` and `shopware.ssl.proxy.enable` are both set.
          Only one of the two may be set. Disable one of the two options.
        '';
      }
    ];

    env.APP_ENV = mkDefault "dev";

    env.APP_URL = mkDefault "${cfg.protocol}://${cfg.domain}:${toString (if cfg.ssl.proxy.enable then cfg.ssl.proxy.port else cfg.port)}";
    env.APP_URL_HTTP = optionalEnv cfg.ssl.standalone.enable "http://${cfg.domain}:${toString cfg.ssl.standalone.fallbackPort}";
    
    env.STOREFRONT_PATH = mkDefault "${config.env.DEVENV_ROOT}/src/Storefront/Resources/app/storefront";
    env.ADMIN_PATH = mkDefault "${config.env.DEVENV_ROOT}/src/Administration/Resources/app/administration";
  };
}
