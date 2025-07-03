{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.blackfire;
in with lib; {
  options.shopware.modules.blackfire = {
    enable = mkOption {
      description = ''
        Enable blackfire agent and configuration.
        Credentials have to be set via `services.blackfire.{client-id,client-token,server-id,server-token}`.
        These can be found at https://blackfire.io/my/settings/credentials.
      '';
      type = types.bool;
      default = false;
    };
    port = mkOption {
      description = "Port for blackfire's socket to listen on.";
      type = types.port;
      default = config.shopware.port + 11;
    };
  };

  config = mkIf cfg.enable {
    languages.php.extensions = [ "blackfire" ];
    languages.php.ini = ''
      blackfire.log_file = ${config.env.DEVENV_ROOT}/var/log/blackfire.log
    '';

    services.blackfire.enable = true;
    services.blackfire.socket = "tcp://127.0.0.1:${toString cfg.port}";
  };
}
