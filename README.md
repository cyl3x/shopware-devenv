# shopware-devenv
A little wrapper around the Shopware devenv environment that gets out of your way.

This will help you to daemonize devenv and allow you to run certain commands from anywhere in the project.

## How to install
### Archlinux
- `mkdir /tmp/swde; cd /tmp/swde`
- `wget https://raw.githubusercontent.com/cyl3x/shopware-devenv/master/PKGBUILD`
- `makepkg -si`

### Generic
1. Setup `devenv` with `direnv` ([Shopware docs](https://developer.shopware.com/docs/guides/installation/devenv))
2. Download the pre-compiled binary and put it in:
   - Linux: `~/.local/bin` or `/usr/local/bin`
   - MacOs: `/usr/local/bin`
   - Windows: Currently unsupported

Do not forget to run `swde config` inside your platform project directory

## How to use
```
Wrapper for the Shopware devenv development environment

Usage: swde [OPTIONS] <COMMAND>

Commands:
  comp         Generate a SHELL completion script and print to stdout
  config       Update to the included devenv.local.nix
  up           Start devenv in background
  down         Stop devenv in background
  log          Show devenv logs
  build        Build storefront/admin/platform
  watch        Watch storefront/admin/unit/jest
  console      bin/console from platform
  plugin       Install/Activate/Uninstall/Reinstall plugins
  update       Update platform and garbage collect devenv
  dump-server  Start the symfony dump server
  help         Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Set the verbosity. Can also be set with SWDE_VERBOSE=1
  -h, --help     Print help
  -V, --version  Print version
```

### Storefront proxy
The current configuration of `devenv.local.nix` favours https over http.  
Since the storefront proxy can't proxy https and has no override capabilities, a patch was needed.

The patch is available via `fix-storefront-proxy` and can be reverted with `unfix-storefront-proxy` and must be run once before `composer watch:storefront`.

## Features
- Daemonize `devenv up` - no need for keep this in foreground
- Update your `devenv.local.nix` trough `swde config`
- Symfony dump server support
- Easily install plugins
- Storefront proxy patch for https
- Execute commands everywhere inside the project
  - e.g. `bin/console`
  - e.g. `composer`, limited to the wrappers like `build` and `watch`
- Shell completions
- All tools on one domain

## devenv.local.nix
By simply executing `swde config` a `devenv.local.nix` will be dumped (Your old one will be backed up).

The config will help you to manage multiple concurrent devenv setups.
