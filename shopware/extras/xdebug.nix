{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.extras.xdebug;
in with lib; {
  options.shopware.extras.xdebug = {
    enable = mkOption { type = types.bool; default = true; };
    logging = mkOption { type = types.either types.int types.bool; default = false; };
  };

  config = mkIf cfg.enable {
    languages.php.extensions = [ "xdebug" ];
    languages.php.ini = ''
      xdebug.mode = debug
      xdebug.discover_client_host = 1
      xdebug.client_host = 127.0.0.1
    '' + optionalString (cfg.logging == true || builtins.isInt cfg.logging) ''
      xdebug.log = ${config.env.DEVENV_ROOT}/var/log/xdebug.log
      xdebug.log_level = ${if builtins.isInt cfg.logging then cfg.logging else 3}
      xdebug.output_dir = ${config.env.DEVENV_ROOT}/var/log/xdebug
    '';
  };
}
