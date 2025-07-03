{ config, ... }@args: let
  lib = import ./lib.nix args;

  cfg = config.shopware;
in with lib; {
  # ordering is important, as last config comes first
  # caddyfile needs to be ordered
  imports = [
    ./ssl.nix
    ./scripts.nix

    ./modules/admin-watcher.nix
    ./modules/adminer.nix
    ./modules/blackfire.nix
    ./modules/cypress.nix
    ./modules/elasticsearch.nix
    ./modules/mailpit.nix
    ./modules/messenger.nix
    ./modules/mysql.nix
    ./modules/playwright.nix
    ./modules/rabbitmq.nix
    ./modules/redis.nix
    ./modules/store-watcher.nix
    ./modules/var-dump-server.nix
    ./modules/xdebug.nix

    ./extras/monolog.nix
    ./extras/start-proxy.nix
    ./extras/vscode-workspace.nix

    ./caddy.nix
  ];

  options.shopware = {
    enable = mkOption {
      description = "Enable and configure all Shopware related services.";
      type = types.bool;
      default = false;
    };
    domain = mkOption {
      description = ''
      The domain on which Shopware will be available.
      A subdomain of `localhost` will be derived based on the directory name.
      '';
      type = types.str;
      default = let
        basename = strings.removePrefix "platform_" (builtins.baseNameOf config.env.DEVENV_ROOT);
        subdomain = if basename == "platform" then "trunk" else basename;
      in subdomain + ".localhost";
      defaultText = "<part-of-dirname>.localhost";
    };
    port = mkOption {
      description = "The base port on which Shopware will be available. All sub services will be derived from this port.";
      type = types.port;
      default = 3000;
    };

    protocol = mkOption {
      description = "Determines the protocol used to access Shopware.";
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

    process.manager.implementation = lib.mkDefault "process-compose";
    devenv.warnOnNewVersion = false;

    env.APP_URL = mkDefault "${cfg.protocol}://${cfg.domain}:${toString (if cfg.ssl.proxy.enable then cfg.ssl.proxy.port else cfg.port)}";
    env.APP_URL_HTTP = let
      enable = cfg.ssl.standalone.enable || cfg.ssl.proxy.enable;
      port = if cfg.ssl.standalone.enable then cfg.ssl.standalone.fallbackPort else cfg.port;
    in optionalEnv enable "http://${cfg.domain}:${toString port}";
    
    env.STOREFRONT_PATH = mkDefault "${config.env.DEVENV_ROOT}/src/Storefront/Resources/app/storefront";
    env.ADMIN_PATH = mkDefault "${config.env.DEVENV_ROOT}/src/Administration/Resources/app/administration";

    scripts."update-module".exec = (builtins.readFile ../update_modules.bash) + ''
      update_module '${config.env.DEVENV_ROOT}/devenv.local.nix'
    '';
  };
}
