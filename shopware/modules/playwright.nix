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
    enable = mkOption { type = types.bool; default = false; };
    chromium = mkOption { type = types.bool; default = true; };
    firefox = mkOption { type = types.bool; default = false; };
    webkit = mkOption { type = types.bool; default = false; };
  };

  config = mkIf cfg.enable {
    env.PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";
    env.PLAYWRIGHT_BROWSERS_PATH = "${playwright-browsers}";
    env.PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS = "1";
  };
}
