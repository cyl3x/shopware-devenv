{ config, lib, ... }:
rec {
  inherit (lib) mkAfter mkBefore mkMerge mkOption types strings optionalString lists attrsets;

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
  
  # Runs `exec` before any process is started.
  # The native process manager does not support `process.manager.before`, a task is used instead.
  # `after` orders the task after other `shopware:<name>` tasks, `order` (mkBefore/mkAfter) does the same for `process.manager.before`.
  mkBeforeProcesses = { name, exec, after ? [], order ? (x: x) }: let
    isNative = config.process.manager.implementation == "native";
  in mkMerge [
    (lib.mkIf isNative {
      tasks."shopware:${name}" = {
        inherit exec;
        after = map (task: "shopware:${task}") after;
        before = lib.mapAttrsToList (process: _: "devenv:processes:${process}") config.processes;
      };
    })
    (lib.mkIf (!isNative) {
      process.manager.before = order exec;
    })
  ];

  mkIf = c: lib.mkIf (c && config.shopware.enable);

  mkMergeIf = con: list: mkIf con (mkMerge list);

  compareSWVersion = version: builtins.compareVersions version config.shopware.version;
}