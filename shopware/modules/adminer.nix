{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.adminer;
in with lib; {
  options.shopware.modules.adminer = {
    enable = mkOption { type = types.bool; default = true; };
    domain = mkOption {
      readOnly = true;
      type = types.str;
      default = "adminer.${config.shopware.domain}";
    };
    port = mkOption {
      readOnly = true;
      type = types.port;
      default = config.shopware.port + 3;
    };
  };

  config = mkMergeIf cfg.enable [
    (mkCaddyProxy {
      inherit (cfg) domain port;
    })

    {
      services.adminer.enable = mkDefault true;
      services.adminer.listen = mkDefault "127.0.0.1:${toString cfg.port}";
    }
  ];
}
