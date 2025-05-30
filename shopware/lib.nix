{ config, lib, ... }:
rec {
  inherit (lib) mkAfter mkMerge mkOption types strings optionalString lists attrsets;

  mkDefault = lib.mkOverride 900;

  optionalEnv = condition: value: mkDefault (if condition then value else null);

  mkCaddyProxy = {
    domain,
    port,
    http ? (!config.shopware.ssl.proxy.enable),
    https ? config.shopware.ssl.standalone.enable,
    host ? "127.0.0.1",
  }: let
    swPort = if config.shopware.ssl.standalone.enable
      then config.shopware.ssl.standalone.fallbackPort
      else config.shopware.port;
  in {
    services.caddy.config = (lib.optionalString http ''
      http://${domain}:${toString swPort} {
        reverse_proxy http://${host}:${toString port}
      }
    '') + (lib.optionalString https ''
      https://${domain}:${toString config.shopware.port} {
        reverse_proxy http://${host}:${toString port}
      }
    '');
  };
  
  mkIf = c: lib.mkIf (c && config.shopware.enable);

  mkMergeIf = con: list: mkIf con (mkMerge list);
}