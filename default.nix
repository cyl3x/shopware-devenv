{ config, ... }:
{
  imports = [
    ./shopware/default.nix
    ./shopware-proxy.nix
  ];

  assertions = [
    {
      assertion = !(config.shopware.enable && config.shopware-proxy.enable);
      message = ''
        `shopware.enable` and `shopware-proxy.enable` are both set.
        Only one of the two may be set. Disable one of the two options.
        If you wish to have https may set `shopware.ssl.standalone.enable`
      '';
    }
  ];
}