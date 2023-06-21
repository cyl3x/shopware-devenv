# shopware-devenv
A little wrapper around the Shopware devenv environment.

This will help you to daemonize devenv and allow you to run certain commands from anywhere in the project.

## How to install
1. Setup `devenv` with `direnv` ([Shopware docs](https://developer.shopware.com/docs/guides/installation/devenv))
2. Download the pre-compiled binary and put it in:
   - Linux: `~/.local/bin` or `/usr/local/bin`
   - MacOs: `/usr/local/bin`
   - Windows: Currently unsupported

## How to use
```
Wrapper for the Shopware devenv development environment

Usage: swde [OPTIONS] <COMMAND>

Commands:
  comp         Generate a SHELL completion script and print to stdout
  config       Update to the included `devenv.local.nix`
  up           Start devenv in background
  down         Stop devenv in background
  log          Show devenv logs
  build        Build storefront/admin/platform
  check        Check code for ci issues
  watch        Watch storefront/admin/unit/jest
  console      bin/console from platform
  plugin       Install/Activate/Uninstall/Reinstall plugins
  update       Update platform and garbage collect devenv
  dump-server  Start the symfony dump server
  help         Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Set the verbosity
  -h, --help     Print help
  -V, --version  Print version
```

There's also an environment variable `SWDE_VERBOSE` to toggle the verbosity.

## devenv.local.nix
The binary includes a `devenv.local.nix` which will help you to manage multiple concurrent devenv setups. Simply execute `swde init`
