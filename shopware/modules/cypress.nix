{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.cypress;
in with lib; {
  options.shopware.modules.cypress = {
    enable = mkOption {
      description = "Enable cypress and necessary configuration.";
      type = types.bool;
      default = false;
    };
  };

  config = mkIf cfg.enable {
    env.CYPRESS_baseUrl = mkDefault config.env.APP_URL;
    env.CYPRESS_dbHost = mkDefault "127.0.0.1:${toString config.shopware.modules.mysql.port}";
    env.CYPRESS_localUsage = mkDefault 1;
    env.CYPRESS_shopwareRoot = mkDefault config.env.DEVENV_ROOT;

    packages = [ pkgs.cypress ];
  };
}
