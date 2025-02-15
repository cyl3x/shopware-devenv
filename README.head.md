# Devenv modules

Set of modules to quickly configure a Shopware development environment:
- Launch multiple instances simultaneously without port conflicts
- Use self-signed certificates for https, including trusting them in your environment
- Use an SSL proxy to share trusted certificates with platform projects and application servers simultaneously
- QoL enhancements and pre-configured tools

## How to use
```nix
{ ... }: let
  modules = builtins.fetchGit {
    url = "https://github.com/cyl3x/shopware-devenv";
    ref = "devenv";
    rev = "<current-commit-sha>";
  };
in {
  imports = [modules.outPath];

  shopware.enable = true;
  shopware.port = 3000;
  shopware.ssl.standalone.enable = true;
}

```

# Options

List of all available configurations with this module set.
