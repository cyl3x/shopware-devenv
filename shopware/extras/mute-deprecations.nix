{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.extras.mute-deprecations;
in with lib; {
  options.shopware.extras.mute-deprecations = {
    enable = mkOption {
      description = "Enable to mute deprecation notices in logs.";
      type = types.bool;
      default = false;
    };
  };

  config = mkIf cfg.enable {
    languages.php.ini = lib.mkAfter ''
      error_reporting = E_ALL ^ E_DEPRECATED
    '';

    files."config/packages/_devenv.yaml".yaml = {
      "when@dev".monolog = {
        channels = [ "deprecation" ];
        handlers = {
          main.channels = [ "!event" "!deprecation" ];
          deprecation = {
            type = "null";
            channels = [ "deprecation" ];
          };
          console = {
            type = "console";
            channels = [ "!doctrine" "!deprecation" ];
          };
        };
      };
    };
  };
}
