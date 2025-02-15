## shopware\.modules\.admin-watcher\.domain

Domain on which the admin-watcher is available\.



*Type:*
string *(read only)*



*Default:*
` "admin..localhost" `

*Declared by:*
 - [\./shopware/modules/admin-watcher\.nix](./shopware/modules/admin-watcher.nix)



## shopware\.modules\.admin-watcher\.port



Port on which the admin-watcher is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3001 `

*Declared by:*
 - [\./shopware/modules/admin-watcher\.nix](./shopware/modules/admin-watcher.nix)



## shopware\.modules\.adminer\.domain



Domain on which the adminer is available\.



*Type:*
string *(read only)*



*Default:*
` "adminer..localhost" `

*Declared by:*
 - [\./shopware/modules/adminer\.nix](./shopware/modules/adminer.nix)



## shopware\.modules\.adminer\.port



Port on which the adminer is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3003 `

*Declared by:*
 - [\./shopware/modules/adminer\.nix](./shopware/modules/adminer.nix)



## shopware\.modules\.elasticsearch\.port



Port on which elasticsearch is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3012 `

*Declared by:*
 - [\./shopware/modules/elasticsearch\.nix](./shopware/modules/elasticsearch.nix)



## shopware\.modules\.elasticsearch\.tcp-port



TCP port on which elasticsearch is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3013 `

*Declared by:*
 - [\./shopware/modules/elasticsearch\.nix](./shopware/modules/elasticsearch.nix)



## shopware\.modules\.mailpit\.domain



Domain on which mailpit is available\.



*Type:*
string *(read only)*



*Default:*
` "mailpit..localhost" `

*Declared by:*
 - [\./shopware/modules/mailpit\.nix](./shopware/modules/mailpit.nix)



## shopware\.modules\.mailpit\.port



Port on which mailpit is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3007 `

*Declared by:*
 - [\./shopware/modules/mailpit\.nix](./shopware/modules/mailpit.nix)



## shopware\.modules\.mailpit\.smtp-port



Port on which mailpit listens for SMTP connections\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3008 `

*Declared by:*
 - [\./shopware/modules/mailpit\.nix](./shopware/modules/mailpit.nix)



## shopware\.modules\.mysql\.port



Port on which mysql is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3006 `

*Declared by:*
 - [\./shopware/modules/mysql\.nix](./shopware/modules/mysql.nix)



## shopware\.modules\.rabbitmq\.domain



Domain on which rabbitmq management is available\.



*Type:*
string *(read only)*



*Default:*
` "rabbitmq..localhost" `

*Declared by:*
 - [\./shopware/modules/rabbitmq\.nix](./shopware/modules/rabbitmq.nix)



## shopware\.modules\.rabbitmq\.management-port



Port on which rabbitmq management is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3011 `

*Declared by:*
 - [\./shopware/modules/rabbitmq\.nix](./shopware/modules/rabbitmq.nix)



## shopware\.modules\.rabbitmq\.port



Port on which rabbitmq is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3010 `

*Declared by:*
 - [\./shopware/modules/rabbitmq\.nix](./shopware/modules/rabbitmq.nix)



## shopware\.modules\.redis\.port



Port on which the redis is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3005 `

*Declared by:*
 - [\./shopware/modules/redis\.nix](./shopware/modules/redis.nix)



## shopware\.modules\.store-watcher\.asset-port



Port on which the store-watcher serves assets\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3004 `

*Declared by:*
 - [\./shopware/modules/store-watcher\.nix](./shopware/modules/store-watcher.nix)



## shopware\.modules\.store-watcher\.domain



Domain on which the store-watcher is available\.



*Type:*
string *(read only)*



*Default:*
` "store..localhost" `

*Declared by:*
 - [\./shopware/modules/store-watcher\.nix](./shopware/modules/store-watcher.nix)



## shopware\.modules\.store-watcher\.port



Port on which the store-watcher is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3002 `

*Declared by:*
 - [\./shopware/modules/store-watcher\.nix](./shopware/modules/store-watcher.nix)



## shopware\.modules\.var-dump-server\.port



Port on which the var dumper is available\.



*Type:*
16 bit unsigned integer; between 0 and 65535 (both inclusive) *(read only)*



*Default:*
` 3009 `

*Declared by:*
 - [\./shopware/modules/var-dump-server\.nix](./shopware/modules/var-dump-server.nix)



## shopware\.protocol



Determines the protocol used to access Shopware\.



*Type:*
one of “http”, “https” *(read only)*



*Default:*
` "http" `

*Declared by:*
 - [\./shopware/default\.nix](./shopware/default.nix)



## shopware\.ssl\.proxy\.rootCA



Path to the root certificate of the proxy\.



*Type:*
string *(read only)*



*Default:*
` "/../.devenv/state/caddy/data/caddy/pki/authorities/local/root.crt" `

*Declared by:*
 - [\./shopware/ssl\.nix](./shopware/ssl.nix)


