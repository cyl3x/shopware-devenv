{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.extras.xdebug;
in with lib; {
  options.shopware.extras.xdebug = {
    enable = mkOption { type = types.bool; default = true; };
  };

  config = mkIf cfg.enable {
    languages.php.extensions = [ "xdebug" ];
    languages.php.ini = ''
      xdebug.mode = debug
      xdebug.discover_client_host = 1
      xdebug.client_host = 127.0.0.1
    '';
  };
}
