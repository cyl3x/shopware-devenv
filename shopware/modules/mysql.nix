{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.mysql;
in with lib; {
  options.shopware.modules.mysql = {
    enable = mkOption {
      description = "Enable mysql and necessary configuration.";
      type = types.bool;
      default = true;
    };
    port = mkOption {
      description = "Port on which mysql is available.";
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 6;
    };
  };

  config = mkIf cfg.enable {
    env.DATABASE_URL = mkDefault "mysql://shopware:shopware@127.0.0.1:${toString cfg.port}/shopware?sslmode=disable&charset=utf8mb4";

    services.mysql.settings.mysqld.port = mkDefault cfg.port;
  };
}
