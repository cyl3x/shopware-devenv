{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.extras.start-proxy;
in with lib; {
  options.shopware.extras.start-proxy = {
    enable = mkOption { type = types.bool; default = config.shopware.ssl.proxy.enable; };
  };

  config = mkIf cfg.enable {
    process.manager.before = ''
      if [ ! -f "${config.shopware.ssl.proxy.devenv}/processes.pid" ] || ! ps -p "$(cat "${config.shopware.ssl.proxy.devenv}/processes.pid")" > /dev/null; then
        oldPwd="$PWD"
        cd "${config.shopware.ssl.proxy.devenv}/.."
        devenv up -d || true
        cd "$oldPwd"
      fi
    '';
  };
}
