{ config, pkgs, ... }@args: let
  lib = import ../lib.nix args;
  cfg = config.shopware.extras.vscode-workspace;
in with lib; {
  options.shopware.extras.vscode-workspace = {
    enable = mkOption {
      description = ''
        Enable to generate a vscode workspace file.
        It also includes all plugins, to regenerate the file use `vscode-ws`
      '';
      type = types.bool;
      default = false;
    };
    excludedPlugins = mkOption {
      description = "List of plugin folder names exlcuded from the workspace";
      type = types.listOf types.str;
      default = [];
    };
  };

  config.scripts = lib.mkIf cfg.enable {
    vscode-ws.exec = let
      filename = "${builtins.baseNameOf config.env.DEVENV_ROOT}.code-workspace";

      base-config = {
        folders = [{ path = "."; }];

        settings = {
          "intelephense.completion.fullyQualifyGlobalConstantsAndFunctions" = true;
          "intelephense.rename.namespaceMode" = "all";
          "intelephense.phpdoc.returnVoid" = false;
          "intelephense.compatibility.preferPsalmPhpstanPrefixedAnnotations" = true;

          "errorLens.enabledInMergeConflict" = false;

          "files.associations" = {
            "**/Resources/app/administration/**/*.html.twig" = "vue-html";
          };

          "files.exclude" = {
            "**/.tmp/**" = true;
            "**/.vite/**" = true;
            "public/**/*[!.php]" = true;
          };

          "search.exclude" = {
            "**/vendor" = true;
            "**/vendor-bin" = true;
            "**/node_modules" = true;
            "**.log" = true;
            "**/.devenv" = true;
            "**/.direnv" = true;
            "**/var/cache" = true;
            "**.lock" = true;
            "**/package-lock.json" = true;
            "**/public/administration/**" = true;
            "**/public/static/**" = true;
          };

          "files.eol" = "\n";
          "files.trimTrailingWhitespace" = true;
          "editor.autoIndent" = "advanced";

          "git.repositoryScanIgnoredFolders" = [ "node_modules" "vendor" ];
          "todo-tree.tree.scanMode" = "workspace";

          "emmet.includeLanguages".vue-html = "html";
        };

        extensions.recommendations = [
          # https://github.com/QISCT/symfony-vscode
          # basic language support
          "mkhl.direnv" # direnv/devenv support
          "redhat.vscode-xml" # xml support
          "redhat.vscode-yaml" # yaml support
          "mrmlnc.vscode-scss" # scss support
          "bmewburn.vscode-intelephense-client" # php support
          "Vue.volar" # vue support
          "Vue.vscode-typescript-vue-plugin" # typescript support
          "mikestead.dotenv" # .env support
          "EditorConfig.EditorConfig" # .editorconfig support

          # extended php
          "SanderRonde.phpstan-vscode"
          "xdebug.php-debug"
          "recca0120.vscode-phpunit"
          "MehediDracula.php-namespace-resolver"

          # extended javascript
          "Orta.vscode-jest"
          "dbaeumer.vscode-eslint"
          "steoates.autoimport"

          # snippets
          "phiter.phpstorm-snippets"
          "onecentlin.phpunit-snippets"
          "sdras.vue-vscode-snippets"
          "xabikos.JavaScriptSnippets"

          # QOL
          "usernamehw.errorlens"
          "RobertOstermann.inline-parameters-extended"
          "Gruntfuggly.todo-tree" # highlights some comments, e.g. @todo, @fixme
          "DEVSENSE.composer-php-vscode"
          "codezombiech.gitignore"
          "ryanluker.vscode-coverage-gutters"
        ];

        extensions.unwantedRecommendations = [
          "vscode.typescript-language-features" # covered by "Vue.vscode-typescript-vue-plugin"
          "vscode.php-language-features" # covered by "bmewburn.vscode-intelephense-client"
        ];
      };
    in ''
      find ./custom/plugins -maxdepth 1 -type d ! -name 'plugins' -printf '%f\n' | ${pkgs.jq}/bin/jq -Rn \
        --argjson ws '${builtins.toJSON base-config}' \
        --argjson excludedPlugins '${builtins.toJSON cfg.excludedPlugins}' \
        '$ws | .folders = [inputs | select(. as $i | $excludedPlugins | index($i) | not) | {path: ("custom/plugins/" + .)}] + .folders' > ${filename}
    '';
  };
}