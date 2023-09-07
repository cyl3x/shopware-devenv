{ pkgs, config, lib, ... }:

let vars = {
    # instance number if you want to start multiple projects
    instance = 0;
    base_domain = "dev.localhost";
    base_port = 2000 + vars.instance * 1000; # baseport of the instance
    # prefixes of different services for base_url
    prefix = {
        platform = ""; 
        admin_watcher = "admin.";
        store_watcher = "store.";
        adminer = "adminer.";
        mailpit = "mail.";
    };

    # ports of the different services
    # every service with a web interface is accessable
    # trough $prefix.$base_url:$base_port instead
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
    env.DATABASE_URL = lib.mkForce "mysql://shopware:shopware@127.0.0.1:${toString vars.port.mysql}/shopware?sslmode=disable&charset=utf8mb4";
    env.APP_URL = lib.mkForce "https://${vars.prefix.platform}${vars.base_domain}:${vars.port.platform.https}";
    env.MAILER_DSN = lib.mkForce "smtp://127.0.0.1:${vars.port.mailpit_smtp}";

    # Storefront
    env.PROXY_URL = lib.mkForce "https://${vars.prefix.store_watcher}${vars.base_domain}:${vars.port.platform.https}";
    env.STOREFRONT_PROXY_PORT = lib.mkForce "${vars.port.store_watcher}";
    env.STOREFRONT_ASSETS_PORT = lib.mkForce "${vars.port.store_asset}";

    # ADMIN
    env.HOST = lib.mkForce "${vars.base_domain}";
    env.PORT = lib.mkForce "${vars.port.admin_watcher}";
    env.IPV4FIRST = lib.mkForce "true";

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

    # MySQL
    # services.mysql.package = pkgs.mariadb; # Use MariaDB instead of MySQL
    services.adminer.listen = "localhost:${vars.port.adminer}";
    services.mysql.settings.mysqld.port = vars.port.mysql;

    # Redis
    services.redis.port = vars.port.redis;

    # Caddy
    services.caddy.config = ''
        https://${vars.base_domain}:${vars.port.platform.https}, http://${vars.base_domain}:${vars.port.platform.http} {
            @default {
              not path /theme/* /media/* /thumbnail/* /bundles/* /css/* /fonts/* /js/* /sitemap/*
            }

            root * public
            php_fastcgi @default unix/${config.languages.php.fpm.pools.web.socket} {
                trusted_proxies private_ranges
            }
            file_server
        }

        https://${vars.prefix.store_watcher}${vars.base_domain}:${vars.port.platform.https}, http://${vars.prefix.store_watcher}${vars.base_domain}:${vars.port.platform.http} {
            reverse_proxy http://localhost:${vars.port.store_watcher}
        }

        https://${vars.prefix.admin_watcher}${vars.base_domain}:${vars.port.platform.https}, http://${vars.prefix.admin_watcher}${vars.base_domain}:${vars.port.platform.http} {
            reverse_proxy http://localhost:${vars.port.admin_watcher}
        }

        https://${vars.prefix.adminer}${vars.base_domain}:${vars.port.platform.https}, http://${vars.prefix.adminer}${vars.base_domain}:${vars.port.platform.http} {
            reverse_proxy http://localhost:${vars.port.adminer}
        }

        https://${vars.prefix.mailpit}${vars.base_domain}:${vars.port.platform.https}, http://${vars.prefix.mailpit}${vars.base_domain}:${vars.port.platform.http} {
            reverse_proxy http://localhost:${vars.port.mailpit}
        }
    '';

    # PHP
    languages.php.extensions = [ "xdebug" ];
    languages.php.ini = lib.mkForce ''
        xdebug.mode = debug
        xdebug.discover_client_host = 1
        xdebug.client_host = localhost
        memory_limit = 2G
        realpath_cache_ttl = 3600
        session.gc_probability = 0
        ${lib.optionalString config.services.redis.enable ''
        session.save_handler = redis
        session.save_path = "tcp://localhost:${toString vars.port.redis}/0"
        ''}
        display_errors = On
        error_reporting = E_ALL
        assert.active = 0
        opcache.memory_consumption = 256M
        opcache.interned_strings_buffer = 20
        zend.assertions = 0
        short_open_tag = 0
        zend.detect_unicode = 0
        realpath_cache_ttl = 3600
    '';

    scripts.fix-caddy-cap.exec = ''
        sudo su -c 'find /nix -type f ! -size 0 -executable -name "caddy" -exec setcap CAP_NET_BIND_SERVICE=+eip "{}" \;'
    '';

    env.STOREFRONT_PROXY_PATCH = ''
    diff --git a/src/Storefront/Resources/app/storefront/build/proxy-server-hot/index.js b/src/Storefront/Resources/app/storefront/build/proxy-server-hot/index.js
    index 5f3182a6c8..977509fdee 100644
    --- a/src/Storefront/Resources/app/storefront/build/proxy-server-hot/index.js
    +++ b/src/Storefront/Resources/app/storefront/build/proxy-server-hot/index.js
    @@ -4,7 +4,10 @@
    * is activated or not.
    */
    
    -const { createServer, request } = require('http');
    +// --- THIS CODE IS NOT INDENTED TO BE COMMITTED -- USE unfix-storefront-proxy TO REVERT THIS --- //
    +const { createServer } = require('http');
    +const { request, Agent } = require('https');
    +// --- THIS CODE IS NOT INDENTED TO BE COMMITTED -- USE unfix-storefront-proxy TO REVERT THIS --- //
    const { spawn } = require('child_process');
    
    module.exports = function createProxyServer({ schema, appPort, originalHost, proxyHost, proxyPort, uri }) {
    @@ -37,6 +40,14 @@ module.exports = function createProxyServer({ schema, appPort, originalHost, pro
                        'hot-reload-mode': true,
                        'accept-encoding': 'identity',
                    },
    +// --- THIS CODE IS NOT INDENTED TO BE COMMITTED -- USE unfix-storefront-proxy TO REVERT THIS --- //
    +                agent: new Agent({
    +                    ciphers: 'TLS_AES_128_GCM_SHA256',
    +                    minVersion: 'TLSv1.3',
    +                    maxVersion: 'TLSv1.3',
    +                }),
    +                rejectUnauthorized: false,
    +// --- THIS CODE IS NOT INDENTED TO BE COMMITTED -- USE unfix-storefront-proxy TO REVERT THIS --- //
                };
    
                // Assets
    @@ -76,7 +87,9 @@ module.exports = function createProxyServer({ schema, appPort, originalHost, pro
        }).listen(proxyPort);
    
        // open the browser with the proxy url
    -    openBrowserWithUrl(fullProxyUrl);
    +// --- THIS CODE IS NOT INDENTED TO BE COMMITTED -- USE unfix-storefront-proxy TO REVERT THIS --- //
    +    openBrowserWithUrl(process.env.PROXY_URL ?? fullProxyUrl);
    +// --- THIS CODE IS NOT INDENTED TO BE COMMITTED -- USE unfix-storefront-proxy TO REVERT THIS --- //
    
        return Promise.resolve({ server, proxyUrl: fullProxyUrl });
    };

    '';

    scripts.fix-storefront-proxy.exec = ''
        cd $(git rev-parse --show-toplevel)
        printenv STOREFRONT_PROXY_PATCH | git apply --reject -q -
        rm -f src/Storefront/Resources/app/storefront/build/proxy-server-hot/index.js.rej
    '';

    scripts.unfix-storefront-proxy.exec = ''
        cd $(git rev-parse --show-toplevel)
        printenv STOREFRONT_PROXY_PATCH | git apply -R --reject -q -
        rm -f src/Storefront/Resources/app/storefront/build/proxy-server-hot/index.js.rej
    '';
}
