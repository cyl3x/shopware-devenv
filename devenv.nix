{ config, devenv, lib, pkgs, ... }@inputs: let
  # Recursively filter module options by a predicate.
  # Devenvs own `filterOptions` keeps submodule options unconditionally,
  # which leaks unrelated devenv options (`env`, `git-hooks`, ...) into the docs.
  filterOptions = predicate: options: lib.concatMapAttrs (name: value:
    if lib.isOption value
      then lib.optionalAttrs (predicate value) { ${name} = value; }
      else lib.optionalAttrs (lib.isAttrs value) { ${name} = filterOptions predicate value; }
  ) options;

  eval = pkgs.lib.evalModules {
    modules = [
      "${devenv.outPath}/top-level.nix"
      ./default.nix
      {
        devenv.warnOnNewVersion = false;
        # `devenv.root` is internal without a default, it is normally supplied by the cli.
        devenv.root = config.devenv.root;
      }
    ];
    specialArgs = { inherit pkgs inputs; };
  };

  filteredOptions = filterOptions
    (option: lib.any (lib.hasInfix "/shopware") option.declarations)
    eval.options;

  readonlyOptions = filterOptions
    (option: (builtins.hasAttr "readOnly" option) && option.readOnly)
    filteredOptions;

  writableOptions = filterOptions
    (option: !(builtins.hasAttr "readOnly" option) || !option.readOnly)
    filteredOptions;

  mkDocOptions = optionSet: let
    rewriteSource = decl: let
      path = lib.strings.removePrefix (toString config.devenv.root) (toString decl);
      url = ".${path}";
    in { name = url; url = url; };
  in pkgs.nixosOptionsDoc {
      options = builtins.removeAttrs optionSet [ "_module" ];
      transformOptions = opt: (
        opt // { declarations = map rewriteSource opt.declarations; }
      );
    };
in
{
  options.docs = {
    package = lib.mkOption {
      type = lib.types.package;
      default = (mkDocOptions writableOptions).optionsCommonMark;
    };
    package-readonly = lib.mkOption {
      type = lib.types.package;
      default = (mkDocOptions readonlyOptions).optionsCommonMark;
    };
  };

  config = {
    scripts."generate-docs".exec = ''
      cat ./README.head.md > README.md
      cat ${config.docs.package.outPath} >> README.md
      cat ${config.docs.package-readonly.outPath} > options-readonly.md
    '';
  };
}