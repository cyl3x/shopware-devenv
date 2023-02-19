{ pkgs, config, lib, ... }:

let vars = {
    base_url = "platform.dev.localhost";
    port = {
        platform = {
            http = "3000";
            https = "4000";
        };
        storefront = {
            http = "3001";
            https = "4001";
        };
        admin = {
            http = "3002";
            https = "4002";
        };
    };
}; in {
    # Environment vars
    env.DATABASE_URL = lib.mkForce "mysql://shopware:shopware@127.0.0.1:3306/shopware?sslmode=disable&charset=utf8mb4";
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

    services.caddy.config = ''
        http://${vars.base_url}:${vars.port.platform.http}, https://${vars.base_url}:${vars.port.platform.https} {
             root * public
             php_fastcgi unix/${config.languages.php.fpm.pools.web.socket}
             file_server
        }

        https://${vars.base_url}:${vars.port.storefront.https} {
            reverse_proxy http://localhost:${vars.port.storefront.http}
        }

        https://${vars.base_url}:${vars.port.admin.https} {
            reverse_proxy http://localhost:${vars.port.admin.http}
        }

        https://db.localhost {
            reverse_proxy http://localhost:9080
        }
    '';

    languages.php.extensions = [ "xdebug" ];
    languages.php.ini = ''
        xdebug.mode = debug
        xdebug.discover_client_host = 1
        xdebug.client_host = 127.0.0.1
    '';
}