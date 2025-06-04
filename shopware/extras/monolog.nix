{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.extras.monolog;
in with lib; {
  options.shopware.extras.monolog = {
    enable = mkOption {
      description = "Enable configure monolog, muting or redirecting channels.";
      type = types.bool;
      default = false;
    };
    muted = mkOption {
      description = "List of channels to be muted.";
      type = types.listOf types.str;
      default = [ "cache" "deprecation" "event" "paypal" "request" ];
    };
    extra = mkOption {
      description = "List of channels to be logged in an extra file.";
      type = types.listOf types.str;
      default = [ "paypal" "request" ];
    };
  };

  config = mkIf cfg.enable {
    languages.php.ini = lib.mkAfter ''
      error_reporting = E_ALL ^ E_DEPRECATED
    '';

    files."config/packages/_devenv.yaml".yaml = {
      "when@dev".monolog = {
        channels = [ "deprecation" "paypal" ];
        handlers = {
          main.channels = builtins.map (s: "!${s}") (cfg.muted);
        } // (attrsets.optionalAttrs (lib.lists.any (s: s == "deprecation") cfg.muted) {
          deprecation = {
            type = "null";
            channels = [ "deprecation" ];
          };
          console = {
            type = "console";
            channels = [ "!doctrine" "!deprecation" ];
          };
        }) // (lib.attrsets.genAttrs cfg.extra (c: {
          type = "stream";
          level = "debug";
          channels = [ c ];
          path = "%kernel.logs_dir%/${c}.log";
        }));
      };
    };
  };
}
