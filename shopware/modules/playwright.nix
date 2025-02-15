{ config, pkgs, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.playwright;
  playwright-browsers = pkgs.playwright-driver.passthru.browsers.override {
    withChromium = cfg.chromium;
    withFirefox = cfg.firefox;
    withWebkit = cfg.webkit;
  };
in with lib; {
  options.shopware.modules.playwright = {
    enable = mkOption {
      description = ''
      Enable configuration for playwright. This will download the browsers and set `PLAYWRIGHT_BROWSERS_PATH`.
      Properly only enable if nixos support is needed.
      '';
      type = types.bool;
      default = false;
    };
    chromium = mkOption {
      description = "Enable chromium browser for playwright.";
      type = types.bool;
      default = true;
    };
    firefox = mkOption {
      description = "Enable firefox browser for playwright.";
      type = types.bool;
      default = false;
    };
    webkit = mkOption {
      description = "Enable webkit browser for playwright.";
      type = types.bool;
      default = false;
    };
  };

  config = mkIf cfg.enable {
    env.PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";
    env.PLAYWRIGHT_BROWSERS_PATH = "${playwright-browsers}";
    env.PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS = "1";
  };
}
