{ config, lib, ... }: let
  cfg = config.shopware;
in with lib; {
  config = mkIf cfg.enable {
    services.caddy.enable = true;
    services.caddy.virtualHosts = lib.mkOverride 60 {};
    services.caddy.config = let
      content = ''
        @default {
          not path /theme/* /media/* /thumbnail/* /bundles/* /css/* /fonts/* /js/* /sitemap/*
        }

        file_server
        root * public
        php_fastcgi @default unix/${config.languages.php.fpm.pools.web.socket} {
          trusted_proxies private_ranges
        }
      '';

      port = if cfg.ssl.standalone.enable
        then cfg.ssl.standalone.fallbackPort
        else cfg.port;
    in ''
      {
        auto_https disable_redirects
        skip_install_trust
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
  };
}
