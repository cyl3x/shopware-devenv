{ config, lib, ... }: let
  cfg = config.shopware.scripts;
in with lib; {
  options.shopware.scripts = {
    console = mkOption {
      description = "Wrapper to use `bin/console` in any directory.";
      type = types.bool;
      default = true;
    };
  };

  config.scripts."console" = mkIf cfg.console {
    exec = "${config.env.DEVENV_ROOT}/bin/console \"$@\"";
  };

  config.scripts."update-module".exec = (builtins.readFile ../update_modules.bash) + ''
    update_module ./devenv.local.nix
  '';
}
