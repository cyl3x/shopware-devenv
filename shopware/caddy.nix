{ config, lib, pkgs, ... }: let
  cfg = config.shopware;
in with lib; {
  options.shopware.extras.frankenphp = {
    enable = mkOption {
      description = "Enable FrankenPHP. HIGHLY EXPERIMENTAL!";
      default = false;
    };
  };

  config = mkIf cfg.enable {
    services.caddy.enable = true;
    services.caddy.package = mkIf cfg.extras.frankenphp.enable (pkgs.frankenphp.override {
      php = config.languages.php.package;
    });
    services.caddy.virtualHosts = lib.mkOverride 60 {};
    services.caddy.config = let
      content = ''
        @default {
          not path /theme/* /media/* /thumbnail/* /bundles/* /css/* /fonts/* /js/* /sitemap/*
        }

        file_server
        root * public
      '' + (if cfg.extras.frankenphp.enable then ''
        php_server @default {
          trusted_proxies private_ranges
          file_server off
        }
      '' else ''
        php_fastcgi @default unix/${config.languages.php.fpm.pools.web.socket} {
          trusted_proxies private_ranges
        }
      '');

      port = if cfg.ssl.standalone.enable
        then cfg.ssl.standalone.fallbackPort
        else cfg.port;
    in ''
      {
        auto_https disable_redirects
        skip_install_trust
        ${lib.optionalString cfg.extras.frankenphp.enable "frankenphp"}
      }

      http://${cfg.domain}:${toString port} {
        ${content}
      }
    '' + (lib.optionalString cfg.ssl.standalone.enable ''
      https://${cfg.domain}:${toString cfg.port} {
        ${content}
      }
    '');

    files."config/packages/_devenv.yaml".yaml = lib.optionalAttrs cfg.ssl.proxy.enable {
      framework.trusted_proxies = mkDefault "127.0.0.1";
    };

    scripts."caddy-setcap".exec = ''sudo setcap CAP_NET_BIND_SERVICE=+eip "${config.services.caddy.package}/bin/caddy"'';
  };
}
