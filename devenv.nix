{ config, devenv, lib, pkgs, ... }@inputs: let
  filterOptions = import "${devenv.outPath}/../../filterOptions.nix" lib;

  eval = pkgs.lib.evalModules {
    modules = [
      "${devenv.outPath}/top-level.nix"
      ./default.nix
      { devenv.warnOnNewVersion = false; }
    ];
    specialArgs = { inherit pkgs inputs; };
  };

  filteredOptions = filterOptions
    (path: option: lib.any (lib.hasInfix "/shopware") option.declarations)
    eval.options;

  readonlyOptions = filterOptions
    (path: option: (builtins.hasAttr "readOnly" option) && option.readOnly)
    filteredOptions;

  writableOptions = filterOptions
    (path: option: !(builtins.hasAttr "readOnly" option) || !option.readOnly)
    filteredOptions;

  mkDocOptions = optionSet: let
    rewriteSource = decl: let
      prefix = lib.strings.concatStringsSep "/" (lib.lists.take 4 (lib.strings.splitString "/" decl));
      path = lib.strings.removePrefix prefix decl;
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