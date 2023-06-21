{ pkgs, config, lib, ... }:

let vars = {
    # increment to avoid port conflicts for parallel instances
    # e.g. trunk -> 0, 6.4 -> 1 - this will result in ports 31xx, 41xx
    instance = 0;
    base_url = "platform.dev.localhost";
    port = {
        platform = {
            http = toString (3000 + vars.instance * 100);
            https = toString (4000 + vars.instance * 100);
        };
        storefront = {
            http = toString (3001 + vars.instance * 100);
            https = toString (4001 + vars.instance * 100);
        };
        admin = {
            http = toString (3002 + vars.instance * 100);
            https = toString (4002 + vars.instance * 100);
        };
        adminer = {
            http = toString (3003 + vars.instance * 100);
            https = toString (4003 + vars.instance * 100);
        };
        mailhog = {
            smtp = toString (1025 + vars.instance);
            http = toString (3004 + vars.instance);
            https = toString (4004 + vars.instance);
        };
        mysql = 3306 + vars.instance;
        redis = 6379 + vars.instance;
    };
}; in {
    # Environment vars
    env.DATABASE_URL = lib.mkForce "mysql://shopware:shopware@127.0.0.1:${toString vars.port.mysql}/shopware?sslmode=disable&charset=utf8mb4";
    env.APP_URL = lib.mkForce "http://${vars.base_url}:${vars.port.platform.http}";
    env.CYPRESS_baseUrl = lib.mkForce "http://${vars.base_url}:${vars.port.platform.http}";
    env.NODE_TLS_REJECT_UNAUTHORIZED = lib.mkForce 1;

    # Storefront
    env.PROXY_URL = lib.mkForce "https://${vars.base_url}:${vars.port.storefront.https}";
    env.STOREFRONT_PROXY_PORT = lib.mkForce "${vars.port.storefront.http}";

    # ADMIN
    env.HOST = lib.mkForce "${vars.base_url}";
    env.PORT = lib.mkForce "${vars.port.admin.http}";
    env.IPV4FIRST = lib.mkForce "true";

    # Mailhog
    env.MAILER_DSN = lib.mkForce "smtp://127.0.0.1:${vars.port.mailhog.smtp}";
    env.MH_SMTP_BIND_ADDR = "127.0.0.1:${vars.port.mailhog.smtp}";
    env.MH_API_BIND_ADDR = "127.0.0.1:${vars.port.mailhog.http}";
    env.MH_UI_BIND_ADDR = "127.0.0.1:${vars.port.mailhog.http}";

    # MySQL
    # services.mysql.package = pkgs.mariadb; # Use MariaDB instead of MySQL
    services.adminer.listen = "${vars.base_url}:${vars.port.adminer.http}";
    services.mysql.settings = {
        mysqld = {
            port = vars.port.mysql;
        };
    };

    # Redis
    services.redis.port = vars.port.redis;

    # Caddy
    services.caddy.config = ''
        http://${vars.base_url}:${vars.port.platform.http}, https://${vars.base_url}:${vars.port.platform.https} {
            root * public
            php_fastcgi @default unix/${config.languages.php.fpm.pools.web.socket} {
                trusted_proxies private_ranges
            }
            file_server
        }

        https://${vars.base_url}:${vars.port.storefront.https} {
            reverse_proxy http://127.0.0.1:${vars.port.storefront.http}
        }

        https://${vars.base_url}:${vars.port.admin.https} {
            reverse_proxy http://127.0.0.1:${vars.port.admin.http}
        }

        https://${vars.base_url}:${vars.port.adminer.https} {
            reverse_proxy http://${vars.base_url}:${vars.port.adminer.http}
        }

        https://${vars.base_url}:${vars.port.mailhog.https} {
            reverse_proxy http://127.0.0.1:${vars.port.mailhog.http}
        }
    '';

    # PHP
    languages.php.extensions = [ "xdebug" ];
    languages.php.ini = ''
        xdebug.mode = debug
        xdebug.discover_client_host = 1
        xdebug.client_host = 127.0.0.1
        ${lib.optionalString config.services.redis.enable ''
        session.save_handler = redis
        session.save_path = "tcp://127.0.0.1:${toString vars.port.redis}/0"
        ''}
    '';
}
