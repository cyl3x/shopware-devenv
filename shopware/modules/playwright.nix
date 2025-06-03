{ config, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.modules.playwright;
in with lib; {
  options.shopware.modules.playwright = {
    enable = mkOption {
      description = ''
      Enable configuration for playwright. A `playwright` command will be available.
      Use for systems like NixOS, where `npx run playwight install` not work. 
      '';
      type = types.bool;
      default = false;
    };
  };

  config.scripts = mkIf cfg.enable {
    "playwright".exec = ''
      export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD="1";
      export PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS="1";
      export PLAYWRIGHT_SKIP_BROWSER_GC="1"
      version="$(jq -r '.packages["node_modules/@playwright/test"].version' package-lock.json)"
      export PLAYWRIGHT_BROWSERS_PATH="$(nix build --print-out-paths --no-link "github:pietdevries94/playwright-web-flake/$version#playwright-driver.browsers")"
      npx playwright "$@"
    '';
  };
}
