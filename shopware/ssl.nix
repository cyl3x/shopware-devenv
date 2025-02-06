{ config, lib, pkgs, ... }: let
  cfg = config.shopware.ssl;
in with lib; {
  options.shopware.ssl = {
    standalone = {
      enable = mkOption { type = types.bool; default = false; };
      fallbackPort = mkOption { type = types.nullOr types.port; default = config.shopware.port + 80; }; # http fallback port
    };

    proxy = {
      enable = mkOption { type = types.bool; default = false; };
      port = mkOption { type = types.nullOr types.port; default = 2000; }; # http fallback port
      devenv = mkOption { type = types.str; default = "${config.env.DEVENV_ROOT}/../.devenv"; };
      rootCA = mkOption {
        readOnly = true;
        type = types.str;
        default = "${cfg.proxy.devenv}/state/caddy/data/caddy/pki/authorities/local/root.crt";
      };
    };

    enableCerts = mkEnableOption name // {
      default = cfg.standalone.enable || cfg.proxy.enable;
    };
  };

  config = mkIf cfg.enableCerts (let
    rootCA = if cfg.standalone.enable
      then "${config.env.DEVENV_STATE}/caddy/data/caddy/pki/authorities/local/root.crt"
      else if cfg.proxy.enable
      then cfg.proxy.rootCA
      else null;
    system = if pkgs.stdenv.isLinux then "/etc/ssl/certs/ca-certificates.crt" else "/etc/ssl/cert.pem";
    combined = "${config.env.DEVENV_STATE}/ca-certificates.crt";
  in {
    env.NODE_EXTRA_CA_CERTS = mkDefault combined;

    languages.php.ini = ''
      openssl.cafile = ${combined}
    '';

    process.manager.before = ''
      cat ${escapeShellArg system} ${escapeShellArg rootCA} > ${escapeShellArg combined}
    '';
  });
}
