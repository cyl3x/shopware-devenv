{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.extras.start-proxy;
in with lib; {
  options.shopware.extras.start-proxy = {
    enable = mkOption {
      description = "Enable to start the ssl proxy server together with devenv.";
      type = types.bool;
      default = config.shopware.ssl.proxy.enable;
    };
  };

  # `devenv up -d` exits early if the proxy's processes are already running
  config = mkIf cfg.enable (mkBeforeProcesses {
    name = "start-proxy";
    order = mkBefore;
    exec = ''
      (cd "${config.shopware.ssl.proxy.devenv}/.." && { devenv up -d; devenv processes wait; }) || true
    '';
  });
}
