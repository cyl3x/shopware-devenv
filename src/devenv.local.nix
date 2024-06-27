{ config, lib, pkgs, ... }:

let
    # instance number if you want to start multiple projects
    instance = 0;

    # domains of different services
    domains = rec {
        platform = "dev.localhost";
        admin_watcher = "admin." + platform;
        store_watcher = "store." + platform;
        adminer = "adminer." + platform;
        mailpit = "mail." + platform;
    };

    # baseport calculation of instances
    base_port = 2000 + instance * 1000;
    # ports of the different services
    # every service with a web interface is accessable
    # trough $domains.service:$base_port instead
    # of the ports configured here
    port = {
        platform = {
            https = toString (base_port);
            http = toString (base_port + 80);
        };
        admin_watcher = toString (base_port + 1);
        store_watcher = toString (base_port + 2);
        adminer = toString (base_port + 3);
        store_asset = toString (base_port + 4);
        redis = base_port + 5;
        mysql = base_port + 6;
        mailpit = toString (base_port + 7);
        mailpit_smtp = toString (instance + 8);
        var_dump_server = toString (base_port + 9);
    };

    # combine system and caddy certificates for php trusted certificates
    # useful to proxy an app server with https and trust it's certificates during installation
    certs = {
        combine = false;

        # paths to the certificates
        caddy = config.env.DEVENV_STATE + "/caddy/data/caddy/pki/authorities/local/root.crt";
        system = if pkgs.stdenv.isLinux then "/etc/ssl/certs/ca-certificates.crt" else "/etc/ssl/cert.pem";
        combined = config.env.DEVENV_STATE + "/ca-certificates.crt";
    };

    inherit (lib) escapeShellArg optionalString;
in {
    # Environment vars
    env.DATABASE_URL = "mysql://shopware:shopware@127.0.0.1:${toString port.mysql}/shopware?sslmode=disable&charset=utf8mb4";
    env.APP_URL = "https://${domains.platform}:${port.platform.https}";
    env.APP_URL_HTTP = "http://${domains.platform}:${port.platform.http}";
    env.MAILER_DSN = "smtp://127.0.0.1:${port.mailpit_smtp}";
    env.SHOPWARE_ES_INDEX_PREFIX = "sw-${toString instance}";

    # Storefront
    env.PROXY_URL = "https://${domains.store_watcher}:${port.platform.https}";
    env.STOREFRONT_PROXY_PORT = "${port.store_watcher}";
    env.STOREFRONT_ASSETS_PORT = "${port.store_asset}";
    env.STOREFRONT_PATH = config.env.DEVENV_ROOT + "/src/Storefront/Resources/app/storefront";

    # ADMIN
    env.HOST = "${domains.platform}";
    env.PORT = "${port.admin_watcher}";
    env.ADMIN_PATH = config.env.DEVENV_ROOT + "/src/Administration/Resources/app/administration";

    # CYPRESS
    env.CYPRESS_baseUrl = config.env.APP_URL;
    env.CYPRESS_dbHost = "127.0.0.1:${toString port.mysql}";
    env.CYPRESS_localUsage = 1;
    env.CYPRESS_shopwareRoot = config.env.DEVENV_ROOT;

    # MySQL
    # services.mysql.package = pkgs.mariadb; # Use MariaDB instead of MySQL
    services.adminer.listen = "localhost:${port.adminer}";
    services.mysql.settings.mysqld.port = port.mysql;

    # Mailhog - legacy, replaced by mailpit
    services.mailhog = {
        smtpListenAddress = "127.0.0.1:${port.mailpit_smtp}";
        apiListenAddress = "127.0.0.1:${port.mailpit}";
        uiListenAddress = "127.0.0.1:${port.mailpit}";
    };

    # Mailpit
    services.mailpit = {
        smtpListenAddress = "127.0.0.1:${port.mailpit_smtp}";
        uiListenAddress = "127.0.0.1:${port.mailpit}";
    };

    # Redis
    services.redis.port = port.redis;

    # var-dump Server
    env.VAR_DUMPER_SERVER = "127.0.0.1:${port.var_dump_server}";
    scripts.dump-server.exec = ''
      vendor/bin/var-dump-server --host=127.0.0.1:${port.var_dump_server}
    '';

    # Certificates
    enterShell = optionalString certs.combine ''
      cat ${escapeShellArg certs.system} ${escapeShellArg certs.caddy} > ${escapeShellArg certs.combined}
    '';

    # Caddy
    services.caddy.config = ''
        {
            auto_https disable_redirects
        }

        http://${domains.platform}:${port.platform.http}, https://${domains.platform}:${port.platform.https} {
            @default {
                not path /theme/* /media/* /thumbnail/* /bundles/* /css/* /fonts/* /js/* /sitemap/*
            }

            file_server
            root * public
            php_fastcgi @default unix/${config.languages.php.fpm.pools.web.socket} {
                trusted_proxies private_ranges
            }
        }

        https://${domains.store_watcher}:${port.platform.https}, http://${domains.store_watcher}:${port.platform.http} {
            reverse_proxy http://localhost:${port.store_watcher}
        }

        https://${domains.admin_watcher}:${port.platform.https}, http://${domains.admin_watcher}:${port.platform.http} {
            reverse_proxy http://localhost:${port.admin_watcher}
        }

        https://${domains.adminer}:${port.platform.https}, http://${domains.adminer}:${port.platform.http} {
            reverse_proxy http://localhost:${port.adminer}
        }

        https://${domains.mailpit}:${port.platform.https}, http://${domains.mailpit}:${port.platform.http} {
            reverse_proxy http://localhost:${port.mailpit}
        }
    '';

    # PHP
    languages.php = {
        extensions = [ "xdebug" ];
        ini = ''
            xdebug.mode = debug
            xdebug.discover_client_host = 1
            xdebug.client_host = 127.0.0.1
        '' + optionalString certs.combine ''
            openssl.cafile = ${certs.combined}
        '';
    };

    # Allowes caddy to bind privileged ports (all <1024)
    scripts.fix-caddy-cap.exec = ''
        sudo setcap CAP_NET_BIND_SERVICE=+eip "${config.services.caddy.package}/bin/caddy"
    '';
}
