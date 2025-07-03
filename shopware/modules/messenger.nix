{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.messenger;
in with lib; {
  options.shopware.modules.messenger = {
    enable = mkOption {
      description = "Enable running a message consumer.";
      type = types.bool;
      default = false;
    };
    time-limit = mkOption {
      description = "The time limit in seconds the worker can handle new messages. The messenger will restart after this time.";
      type = types.int;
      default = 300;
    };
    args = mkOption {
      description = "Additional arguments to pass to the messenger consumer.";
      type = types.listOf types.str;
      default = [ "-vv --all" ];
    };
  };

  config = mkIf cfg.enable {
    processes.messenger = {
      exec = "sleep 2 && ${config.env.DEVENV_ROOT}/bin/console messenger:consume --time-limit=${toString cfg.time-limit} ${lib.strings.concatStringsSep " " cfg.args}";
      process-compose = {
        availability = {
          restart = "always";
          max_restarts = 0;
        };
        depends_on.mysql.condition = "process_started";
      };
    };

    files."config/packages/_devenv.yaml".yaml = {
      shopware.admin_worker.enable_admin_worker = false;
    };
  };
}
