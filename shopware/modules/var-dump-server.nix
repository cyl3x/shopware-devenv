{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.var-dump-server;
in with lib; {
  options.shopware.modules.var-dump-server = {
    enable = mkOption {
      description = "Enable configuration for the var-dump-server. Will also create a shotcut `dump-server`.";
      type = types.bool;
      default = true;
    };
    port = mkOption {
      description = "Port on which the var dumper is available.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 9;
    };
  };

  config = mkIf cfg.enable {
    env.VAR_DUMPER_SERVER = mkDefault "127.0.0.1:${toString cfg.port}";

    scripts.dump-server.exec = "${config.env.DEVENV_ROOT}/vendor/bin/var-dump-server --host=127.0.0.1:${toString cfg.port}";

    files."config/packages/_devenv.yaml".yaml = {
      "when@dev" = {
        debug.dump_destination = "tcp://%env(VAR_DUMPER_SERVER)%";
      };
    };
  };
}
