## shopware\.modules\.admin-watcher\.domain

Domain on which the admin-watcher is available\.



*Type:*
string *(read only)*



*Default:*

```nix
"admin.shopware-devenv.localhost"
```

*Declared by:*
 - [\./shopware/modules/admin-watcher\.nix](./shopware/modules/admin-watcher.nix)



## shopware\.modules\.admin-watcher\.port



Port on which the admin-watcher is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3001
```

*Declared by:*
 - [\./shopware/modules/admin-watcher\.nix](./shopware/modules/admin-watcher.nix)



## shopware\.modules\.adminer\.domain



Domain on which the adminer is available\.



*Type:*
string *(read only)*



*Default:*

```nix
"adminer.shopware-devenv.localhost"
```

*Declared by:*
 - [\./shopware/modules/adminer\.nix](./shopware/modules/adminer.nix)



## shopware\.modules\.adminer\.port



Port on which the adminer is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3003
```

*Declared by:*
 - [\./shopware/modules/adminer\.nix](./shopware/modules/adminer.nix)



## shopware\.modules\.elasticsearch\.port



Port on which elasticsearch is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3012
```

*Declared by:*
 - [\./shopware/modules/elasticsearch\.nix](./shopware/modules/elasticsearch.nix)



## shopware\.modules\.elasticsearch\.tcp-port



TCP port on which elasticsearch is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3013
```

*Declared by:*
 - [\./shopware/modules/elasticsearch\.nix](./shopware/modules/elasticsearch.nix)



## shopware\.modules\.javascript\.auto-version



javascript version auto-detected based on the shopware version\.



*Type:*
package *(read only)*



*Default:*

```nix
"<nodejs_22 for 6.4 to 6.6, nodejs_24 for 6.7, nodejs_26 otherwise>"
```

*Declared by:*
 - [\./shopware/modules/javascript\.nix](./shopware/modules/javascript.nix)



## shopware\.modules\.mailpit\.domain



Domain on which mailpit is available\.



*Type:*
string *(read only)*



*Default:*

```nix
"mailpit.shopware-devenv.localhost"
```

*Declared by:*
 - [\./shopware/modules/mailpit\.nix](./shopware/modules/mailpit.nix)



## shopware\.modules\.mailpit\.port



Port on which mailpit is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3007
```

*Declared by:*
 - [\./shopware/modules/mailpit\.nix](./shopware/modules/mailpit.nix)



## shopware\.modules\.mailpit\.smtp-port



Port on which mailpit listens for SMTP connections\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3008
```

*Declared by:*
 - [\./shopware/modules/mailpit\.nix](./shopware/modules/mailpit.nix)



## shopware\.modules\.mysql\.auto-version



MySQL/MariaDB package auto-detected based on the shopware version\.



*Type:*
package *(read only)*



*Default:*

```nix
"<mariadb_106 for 6.4, mariadb_1011 for 6.5, mysql84 otherwise>"
```

*Declared by:*
 - [\./shopware/modules/mysql\.nix](./shopware/modules/mysql.nix)



## shopware\.modules\.mysql\.port



Port on which mysql is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3006
```

*Declared by:*
 - [\./shopware/modules/mysql\.nix](./shopware/modules/mysql.nix)



## shopware\.modules\.php\.auto-version



PHP version auto-detected based on the shopware version\.



*Type:*
string *(read only)*



*Default:*

```nix
"<8.0 for 6.4, 8.2 for 6.5 and 6.6, 8.4 for 6.7, 8.5 otherwise>"
```

*Declared by:*
 - [\./shopware/modules/php\.nix](./shopware/modules/php.nix)



## shopware\.modules\.rabbitmq\.domain



Domain on which rabbitmq management is available\.



*Type:*
string *(read only)*



*Default:*

```nix
"rabbitmq.shopware-devenv.localhost"
```

*Declared by:*
 - [\./shopware/modules/rabbitmq\.nix](./shopware/modules/rabbitmq.nix)



## shopware\.modules\.rabbitmq\.management-port



Port on which rabbitmq management is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3011
```

*Declared by:*
 - [\./shopware/modules/rabbitmq\.nix](./shopware/modules/rabbitmq.nix)



## shopware\.modules\.rabbitmq\.port



Port on which rabbitmq is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3010
```

*Declared by:*
 - [\./shopware/modules/rabbitmq\.nix](./shopware/modules/rabbitmq.nix)



## shopware\.modules\.redis\.port



Port on which the redis is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3005
```

*Declared by:*
 - [\./shopware/modules/redis\.nix](./shopware/modules/redis.nix)



## shopware\.modules\.store-watcher\.asset-port



Port on which the store-watcher serves assets\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3004
```

*Declared by:*
 - [\./shopware/modules/store-watcher\.nix](./shopware/modules/store-watcher.nix)



## shopware\.modules\.store-watcher\.domain



Domain on which the store-watcher is available\.



*Type:*
string *(read only)*



*Default:*

```nix
"store.shopware-devenv.localhost"
```

*Declared by:*
 - [\./shopware/modules/store-watcher\.nix](./shopware/modules/store-watcher.nix)



## shopware\.modules\.store-watcher\.port



Port on which the store-watcher is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3002
```

*Declared by:*
 - [\./shopware/modules/store-watcher\.nix](./shopware/modules/store-watcher.nix)



## shopware\.modules\.var-dump-server\.port



Port on which the var dumper is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*

```nix
3009
```

*Declared by:*
 - [\./shopware/modules/var-dump-server\.nix](./shopware/modules/var-dump-server.nix)



## shopware\.protocol



Determines the protocol used to access Shopware\.



*Type:*
one of “http”, “https” *(read only)*



*Default:*

```nix
"http"
```

*Declared by:*
 - [\./shopware/default\.nix](./shopware/default.nix)



## shopware\.ssl\.proxy\.rootCA



Path to the root certificate of the proxy\.



*Type:*
string *(read only)*



*Default:*

```nix
"<ssl.proxy.devenv>/state/caddy/data/caddy/pki/authorities/local/root.crt"
```

*Declared by:*
 - [\./shopware/ssl\.nix](./shopware/ssl.nix)


