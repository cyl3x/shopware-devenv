## shopware\.enable



Enable and configure all Shopware related services\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/default\.nix](./shopware/default.nix)



## shopware\.domain



The domain on which Shopware will be available\.
A subdomain of ` localhost ` will be derived based on the directory name\.



*Type:*
string



*Default:*
` ".localhost" `

*Declared by:*
 - [\./shopware/default\.nix](./shopware/default.nix)



## shopware\.extras\.start-proxy\.enable



Enable to start the ssl proxy server together with devenv\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/extras/start-proxy\.nix](./shopware/extras/start-proxy.nix)



## shopware\.extras\.xdebug\.enable



Enable Xdebug for PHP\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware/extras/xdebug\.nix](./shopware/extras/xdebug.nix)



## shopware\.extras\.xdebug\.logging



Enable Xdebug logging\. Alternatively, set the log level\.



*Type:*
signed integer or boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/extras/xdebug\.nix](./shopware/extras/xdebug.nix)



## shopware\.modules\.admin-watcher\.enable



Enable configuration for the admin-watcher\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware/modules/admin-watcher\.nix](./shopware/modules/admin-watcher.nix)



## shopware\.modules\.adminer\.enable



Enable and configure adminer\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware/modules/adminer\.nix](./shopware/modules/adminer.nix)



## shopware\.modules\.cypress\.enable



Enable cypress and necessary configuration\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/modules/cypress\.nix](./shopware/modules/cypress.nix)



## shopware\.modules\.elasticsearch\.enable



Enable elasticsearch and necessary configuration\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/modules/elasticsearch\.nix](./shopware/modules/elasticsearch.nix)



## shopware\.modules\.mailpit\.enable



Enable mailpit and necessary configuration\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware/modules/mailpit\.nix](./shopware/modules/mailpit.nix)



## shopware\.modules\.mysql\.enable



Enable mysql and necessary configuration\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware/modules/mysql\.nix](./shopware/modules/mysql.nix)



## shopware\.modules\.playwright\.enable



Enable configuration for playwright\. This will download the browsers and set ` PLAYWRIGHT_BROWSERS_PATH `\.
Properly only enable if nixos support is needed\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/modules/playwright\.nix](./shopware/modules/playwright.nix)



## shopware\.modules\.playwright\.chromium



Enable chromium browser for playwright\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware/modules/playwright\.nix](./shopware/modules/playwright.nix)



## shopware\.modules\.playwright\.firefox



Enable firefox browser for playwright\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/modules/playwright\.nix](./shopware/modules/playwright.nix)



## shopware\.modules\.playwright\.webkit



Enable webkit browser for playwright\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/modules/playwright\.nix](./shopware/modules/playwright.nix)



## shopware\.modules\.rabbitmq\.enable



Enable rabbitmq and necessary configuration\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/modules/rabbitmq\.nix](./shopware/modules/rabbitmq.nix)



## shopware\.modules\.redis\.enable



Enable redis and necessary configuration\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware/modules/redis\.nix](./shopware/modules/redis.nix)



## shopware\.modules\.store-watcher\.enable



Enable configuration for the store-watcher



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware/modules/store-watcher\.nix](./shopware/modules/store-watcher.nix)



## shopware\.modules\.var-dump-server\.enable



Enable configuration for the var-dump-server\. Will also create a shotcut ` dump-server `\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware/modules/var-dump-server\.nix](./shopware/modules/var-dump-server.nix)



## shopware\.port



The base port on which Shopware will be available\. All sub services will be derived from this port\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive)



*Default:*
` 3000 `

*Declared by:*
 - [\./shopware/default\.nix](./shopware/default.nix)



## shopware\.ssl\.enableCerts



Enable creation of a trusted root cert for node\.js and php\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/ssl\.nix](./shopware/ssl.nix)



## shopware\.ssl\.proxy\.enable



Enable ssl via proxy\. Shopware will be available via https by default\. Used domain will taken from ` shopware.domain `\.

No other webserver configuration will be done, it’s assumed that a second devenv instance is running as proxy\. See ` shopware-proxy.enable ` for more information\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/ssl\.nix](./shopware/ssl.nix)



## shopware\.ssl\.proxy\.devenv



Path to the devenv instance that serves as proxy\.



*Type:*
string



*Default:*
` "/../.devenv" `

*Declared by:*
 - [\./shopware/ssl\.nix](./shopware/ssl.nix)



## shopware\.ssl\.proxy\.port



Port on which the proxy runs\.



*Type:*
null or 16 bit unsigned integer; between 0 and 65535 (both inclusive)



*Default:*
` 2000 `

*Declared by:*
 - [\./shopware/ssl\.nix](./shopware/ssl.nix)



## shopware\.ssl\.standalone\.enable



Enable standalone ssl\. Shopware will be available via https by default\. A fallback port can be defined for http\.
Self signed certificates will generated by caddy\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware/ssl\.nix](./shopware/ssl.nix)



## shopware\.ssl\.standalone\.fallbackPort



Fallback port for http



*Type:*
null or 16 bit unsigned integer; between 0 and 65535 (both inclusive)



*Default:*
` 3080 `

*Declared by:*
 - [\./shopware/ssl\.nix](./shopware/ssl.nix)



## shopware-proxy\.enable



Enable the proxy used for shopware and app servers\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware-proxy\.nix](./shopware-proxy.nix)



## shopware-proxy\.apps

Map of app servers to proxy\.



*Type:*
attribute set of 16 bit unsigned integer; between 0 and 65535 (both inclusive)



*Default:*
` { } `

*Declared by:*
 - [\./shopware-proxy\.nix](./shopware-proxy.nix)



## shopware-proxy\.modules\.admin-watcher



Enable reverse proxy for the admin-watcher\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware-proxy\.nix](./shopware-proxy.nix)



## shopware-proxy\.modules\.adminer



Enable reverse proxy for adminer\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware-proxy\.nix](./shopware-proxy.nix)



## shopware-proxy\.modules\.mailpit



Enable reverse proxy for mailpit\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware-proxy\.nix](./shopware-proxy.nix)



## shopware-proxy\.modules\.rabbitmq



Enable reverse proxy for rabbitmq management\.



*Type:*
boolean



*Default:*
` false `

*Declared by:*
 - [\./shopware-proxy\.nix](./shopware-proxy.nix)



## shopware-proxy\.modules\.store-watcher



Enable reverse proxy for the store-watcher\.



*Type:*
boolean



*Default:*
` true `

*Declared by:*
 - [\./shopware-proxy\.nix](./shopware-proxy.nix)



## shopware-proxy\.platforms



Map of platform projects to proxy\.



*Type:*
attribute set of 16 bit unsigned integer; between 0 and 65535 (both inclusive)



*Default:*
` { } `



*Example:*

````
''
  {
    "trunk.localhost" = 3000; # base port of platform, defined by `shopware.port`
    "66.localhost" = 4000;
  }
''
````

*Declared by:*
 - [\./shopware-proxy\.nix](./shopware-proxy.nix)



## shopware-proxy\.port



Port on which the proxy listens\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive)



*Default:*
` 2000 `

*Declared by:*
 - [\./shopware-proxy\.nix](./shopware-proxy.nix)


