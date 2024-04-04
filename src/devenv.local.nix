{ config, ... }:

let vars = {
    # instance number if you want to start multiple projects
    instance = 0;
    base_port = 2000 + vars.instance * 1000; # baseport of the instance

    # domains of different services
    domains = {
        platform = "dev.localhost";
        admin_watcher = "admin.dev.localhost";
        store_watcher = "store.dev.localhost";
        adminer = "adminer.dev.localhost";
        mailpit = "mail.dev.localhost";
    };

    # ports of the different services
    # every service with a web interface is accessable
    # trough $domains.service:$base_port instead
    # of the ports configured here
    port = {
        platform = {
            https = toString (vars.base_port);
            http = toString (vars.base_port + 80);
        };
        admin_watcher = toString (vars.base_port + 1);
        store_watcher = toString (vars.base_port + 2);
        adminer = toString (vars.base_port + 3);
        store_asset = toString (vars.base_port + 4);
        redis = vars.base_port + 5;
        mysql = vars.base_port + 6;
        mailpit = toString (vars.base_port + 7);
        mailpit_smtp = toString (1025 + vars.instance);
    };
}; in {
    # Environment vars
    env.DATABASE_URL = "mysql://shopware:shopware@127.0.0.1:${toString vars.port.mysql}/shopware?sslmode=disable&charset=utf8mb4";
    env.APP_URL = "https://${vars.domains.platform}:${vars.port.platform.https}";
    env.APP_URL_HTTP = "http://${vars.domains.platform}:${vars.port.platform.http}";
    env.MAILER_DSN = "smtp://127.0.0.1:${vars.port.mailpit_smtp}";
    env.SHOPWARE_ES_INDEX_PREFIX = "sw-${toString vars.instance}";

    # Storefront
    env.PROXY_URL = "https://${vars.domains.store_watcher}:${vars.port.platform.https}";
    env.STOREFRONT_PROXY_PORT = "${vars.port.store_watcher}";
    env.STOREFRONT_ASSETS_PORT = "${vars.port.store_asset}";
    env.STOREFRONT_PATH = config.env.DEVENV_ROOT + "/src/Storefront/Resources/app/storefront";

    # ADMIN
    env.HOST = "${vars.domains.platform}";
    env.PORT = "${vars.port.admin_watcher}";
    env.ADMIN_PATH = config.env.DEVENV_ROOT + "/src/Administration/Resources/app/administration";

    # CYPRESS
    env.CYPRESS_baseUrl = config.env.APP_URL;
    env.CYPRESS_dbHost = "127.0.0.1:${toString vars.port.mysql}";
    env.CYPRESS_localUsage = 1;
    env.CYPRESS_shopwareRoot = config.env.DEVENV_ROOT;

    # MySQL
    # services.mysql.package = pkgs.mariadb; # Use MariaDB instead of MySQL
    services.adminer.listen = "localhost:${vars.port.adminer}";
    services.mysql.settings.mysqld.port = vars.port.mysql;

    # Mailhog - legacy, replaced by mailpit
    services.mailhog = {
        smtpListenAddress = "127.0.0.1:${vars.port.mailpit_smtp}";
        apiListenAddress = "localhost:${vars.port.mailpit}";
        uiListenAddress = "localhost:${vars.port.mailpit}";
    };

    # Mailpit
    services.mailpit = {
        smtpListenAddress = "127.0.0.1:${vars.port.mailpit_smtp}";
        uiListenAddress = "127.0.0.1:${vars.port.mailpit}";
    };

    # Redis
    services.redis.port = vars.port.redis;

    # Caddy
    services.caddy.config = ''
        {
            auto_https disable_redirects
        }

        http://${vars.domains.platform}:${vars.port.platform.http}, https://${vars.domains.platform}:${vars.port.platform.https} {
            @default {
            not path /theme/* /media/* /thumbnail/* /bundles/* /css/* /fonts/* /js/* /sitemap/*
            }

            encode zstd gzip
            root * public
            php_fastcgi @default unix/${config.languages.php.fpm.pools.web.socket} {
                trusted_proxies private_ranges
            }
            file_server
            encode

            encode zstd gzip
        }

        https://${vars.domains.store_watcher}:${vars.port.platform.https}, http://${vars.domains.store_watcher}:${vars.port.platform.http} {
            reverse_proxy http://localhost:${vars.port.store_watcher}
        }

        https://${vars.domains.admin_watcher}:${vars.port.platform.https}, http://${vars.domains.admin_watcher}:${vars.port.platform.http} {
            reverse_proxy http://localhost:${vars.port.admin_watcher}
        }

        https://${vars.domains.adminer}:${vars.port.platform.https}, http://${vars.domains.adminer}:${vars.port.platform.http} {
            reverse_proxy http://localhost:${vars.port.adminer}
        }

        https://${vars.domains.mailpit}:${vars.port.platform.https}, http://${vars.domains.mailpit}:${vars.port.platform.http} {
            reverse_proxy http://localhost:${vars.port.mailpit}
        }
    '';

    # PHP
    languages.php.extensions = [ "xdebug" ];
    languages.php.ini = ''
        xdebug.mode = debug
        xdebug.discover_client_host = 1
        xdebug.client_host = 127.0.0.1
    '';
    # Add above to php ini - Helps with installing an app over https, but php will ignore hosts certificates
    # openssl.cafile = ${config.env.DEVENV_STATE + "/caddy/data/caddy/pki/authorities/local/root.crt"}

    # Allowes caddy to bind privileged ports (all <1024)
    scripts.fix-caddy-cap.exec = ''
        sudo setcap CAP_NET_BIND_SERVICE=+eip "${config.services.caddy.package}/bin/caddy"
    '';
}
