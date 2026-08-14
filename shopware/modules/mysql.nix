{ config, pkgs, ... }@args: let
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
    auto-version = mkOption {
      description = "MySQL version & package auto-detected based on the shopware version.";
      readOnly = true;
      type = types.package;
      default = if config.shopware.version == "6.4" then pkgs.mysql80
        else if config.shopware.version == "6.5" then pkgs.mysql80
        else pkgs.mysql84;
      defaultText = "<mysql80 for 6.4 and 6.5, mysql84 otherwise>";
    };
  };

  config = mkIf cfg.enable {
    env.DATABASE_URL = mkDefault "mysql://root@127.0.0.1:${toString cfg.port}/shopware?sslmode=disable&charset=utf8mb4";
    env.DATABASE_URL_TEST = mkDefault "mysql://root@127.0.0.1:${toString cfg.port}/shopware_test?sslmode=disable&charset=utf8mb4";

    services.mysql = {
      enable = true;
      package = lib.mkDefault cfg.auto-version;
      initialDatabases = lib.mkDefault [{ name = "shopware"; }];
      ensureUsers = lib.mkDefault [
        {
          name = "shopware";
          password = "shopware";
          ensurePermissions = {
            "shopware.*" = "ALL PRIVILEGES";
            "shopware_test.*" = "ALL PRIVILEGES";
          };
        }
      ];
      settings = {
        mysqld = {
          port = mkDefault cfg.port;
          group_concat_max_len = 320000;
          log_bin_trust_function_creators = 1;
          sql_mode = "STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION";
        };
      };
    };
  };
}
