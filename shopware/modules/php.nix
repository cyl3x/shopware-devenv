{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.php;
in with lib; {
  options.shopware.modules.php = {
    enable = mkOption {
      description = "Enable php and necessary configuration.";
      type = types.bool;
      default = true;
    };
    auto-version = mkOption {
      description = "PHP version auto-detected based on the shopware version.";
      readOnly = true;
      type = types.str;
      default = if config.shopware.version == "6.4" then "8.0"
        else if config.shopware.version == "6.5" then "8.2"
        else if config.shopware.version == "6.6" then "8.2"
        else if config.shopware.version == "6.7" then "8.4"
        else "8.5";
      defaultText = "<8.0 for 6.4, 8.2 for 6.5 and 6.6, 8.4 for 6.7, 8.5 otherwise>";
    };
  };

  config = mkIf cfg.enable {
    languages.php = {
      enable = true;
      version = mkDefault cfg.auto-version;

      ini = ''
        memory_limit = 2G
        realpath_cache_ttl = 3600
        session.gc_probability = 0
        display_errors = On
        error_reporting = E_ALL
        opcache.memory_consumption = 256M
        opcache.interned_strings_buffer = 20
        zend.assertions = 0
        short_open_tag = 0
        zend.detect_unicode = 0
        realpath_cache_ttl = 3600
        post_max_size = 32M
        upload_max_filesize = 32M
      '';

      fpm.pools.web = mkDefault {
        settings = {
          "clear_env" = "no";
          "pm" = "dynamic";
          "pm.max_children" = 10;
          "pm.start_servers" = 2;
          "pm.min_spare_servers" = 1;
          "pm.max_spare_servers" = 10;
        };
      };
    };
  };
}
