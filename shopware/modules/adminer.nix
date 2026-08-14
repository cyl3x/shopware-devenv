{ config, pkgs, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.adminer;

  # devenv's adminer service uses `<package>/adminer.php` as the php router script, which adminneo does not ship.
  adminneo = pkgs.symlinkJoin {
    name = "adminneo-with-adminer-${pkgs.adminneo.version}";
    paths = [ pkgs.adminneo ];
    postBuild = ''
      echo '<?php require __DIR__ . "/adminneo.php";' > $out/adminer.php
    '';
  };
in with lib; {
  options.shopware.modules.adminer = {
    enable = mkOption {
      description = "Enable and configure adminer.";
      type = types.bool;
      default = true;
    };
    domain = mkOption {
      description = "Domain on which the adminer is available.";
      readOnly = true;
      type = types.str;
      default = "adminer.${config.shopware.domain}";
    };
    port = mkOption {
      description = "Port on which the adminer is available.";
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
      services.adminer.package = mkDefault adminneo;
      services.adminer.listen = mkDefault "127.0.0.1:${toString cfg.port}";
    }
  ];
}
