# Devenv modules

Set of modules to quickly configure a Shopware development environment:
- Launch multiple instances simultaneously without port conflicts
- Use self-signed certificates for https, including trusting them in your environment
- Use an SSL proxy to share trusted certificates with platform projects and application servers simultaneously
- QoL enhancements and pre-configured tools

## How to use
Put this as your `devenv.local.nix`:
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

  # if not set, the domain is derived from the directory name (`platform_` is stripped)
  # shopware.domain = "dev.localhost";
}

```

## How to use (proxy mode)
`devenv.local.nix` inside a platform project:
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
  # Read more about it below under "Options"
  shopware.ssl.proxy.enable = true;
  # Devenv folder of the proxy server. If not set, the parent directory is assumed
  # shopware.ssl.proxy.devenv = "../.devenv"
}

```

`devenv.nix` somewhere, e.g. in a parent directory next to all platform projects and app servers:
```nix
{ ... }: let
  modules = builtins.fetchGit {
    url = "https://github.com/cyl3x/shopware-devenv";
    ref = "devenv";
    rev = "<current-commit-sha>";
  };
in {
  imports = [modules.outPath];

  shopware-proxy = {
    enable = true;
    port = 2000; # port to start poxy on
    platforms = {
      # base port and domain as set in platform directory
      "trunk.localhost" = 3000;
      # some other platform projects
      "65.localhost" = 3100;
      "66.localhost" = 3200;
      "saas.localhost" = 3300;
    };
    apps = {
      "swagbraintree.localhost" = 8080;
    };
  };
}
```

## Utility scripts

- `console` can be used to execute console command in any directory
- `caddy-setcap` can be used to bind privileged ports (like 443). Can only be used on non-nixos systems
- `vscode-ws` can be used to generate a VS Code workspace file. Set `shopware.extras.vscode-workspace.enable = true;`.

# Options

List of all available configurations with this module set.
