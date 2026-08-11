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
    wrapper.phpunit = mkOption {
      description = "Adds a phpunit wrapper for recca0120.vscode-phpunit so it can find phpunit in plugins";
      type = types.bool;
      default = true;
    };
  };

  config = lib.mkIf cfg.enable {
    scripts.vscode-ws.exec = let
      filename = "${builtins.baseNameOf config.env.DEVENV_ROOT}.code-workspace";

      base-config = rec {
        folders = [{ path = "."; }];

        settings = {
          "intelephense.completion.propertyCase" = "camel";
          "intelephense.completion.parameterCase" = "camel";
          "intelephense.completion.fullyQualifyGlobalConstantsAndFunctions" = true;
          "intelephense.rename.namespaceMode" = "all";
          "intelephense.phpdoc.returnVoid" = false;
          "intelephense.compatibility.preferPsalmPhpstanPrefixedAnnotations" = true;
          "intelephense.codeLens.implementations.enable" = true;
          "intelephense.codeLens.overrides.enable" = true;
          "intelephense.codeLens.parent.enable" = true;
          "intelephense.codeLens.references.enable" = true;
          "intelephense.codeLens.usages.enable" = true;
          "intelephense.completion.suggestObjectOperatorStaticMethods" = false;
          "intelephense.files.exclude" = [
            "**/.git/**"
            "**/node_modules/**"
            "**/var/cache/**"
            "**/vendor-bin/**"
            "**/vendor/**/{Tests,tests}/**"
            "**/vendor/**/vendor/**"
            "**/vendor/fakerphp/faker/src/Faker/Provider/*/Text.php"
            "**/vendor/symfony/intl/Resources/data/**"
          ];
          "intelephense.phpdoc.functionTemplate" = {
            "summary" = "$1";
            "tags" = [
              ''@param ''${1:$SYMBOL_TYPE} $SYMBOL_NAME $2''
              ''@return ''${1:$SYMBOL_TYPE} $2''
            ];
          };

          "namespaceResolver.sortAlphabetically" = true;
          "namespaceResolver.sortNatural" = false;

          "errorLens.enabledInMergeConflict" = false;
          "errorLens.excludeBySource" = [ "Harper" ];

          "files.associations" = {
            "**/Resources/app/administration/**/*.html.twig" = "vue-html";
            "*.neon" = "yaml";
            "*.neon.dist" = "yaml";
            "*.php.dist" = "php";
          };

          "files.exclude" = {
            "**/.jestcache" = true;
            "**/.tmp" = true;
            "**/.vite" = true;
            # included via workspace folders. Plugin and app folders will be ~still visible~ not visible (excluding more fine-grained will cause intelephense to malfunction)
            "custom/apps" = true; 
            "custom/plugins" = true;
          };

          "files.watcherExclude" = {
            "**/.devenv/*/**" = true;
            "**/.direnv/**" = true;
            "**/.jestcache/**" = true;
            "**/.tmp/**" = true;
            "**/.vite/**" = true;
            "**/node_modules/**" = true;
            "**/Resources/app/storefront/dist/**" = true;
            "**/Resources/app/storefront/vendor/**" = true;
            "**/Resources/public/**" = true;
            "**/var/cache/**" = true;
            "**/vendor-bin/**" = true;
            "public/bundles/**" = true;
            "public/media/**" = true;
            "public/theme/**" = true;
          };

          "search.exclude" = settings."files.watcherExclude" // settings."files.exclude" // {
            "**.lock" = true;
            "**.log" = true;
            "**/.git/**" = true;
            "**/*.code-search" = true;
            "**/bower_components/**" = true;
            "**/package-lock.json" = true;
            "**/tests/acceptance/test-results/**" = true;
            "**/vendor/**" = true;
          };

          "files.eol" = "\n";
          "files.trimTrailingWhitespace" = true;
          "editor.autoIndent" = "advanced";

          "git.repositoryScanIgnoredFolders" = [ "node_modules" "vendor" ];
          "todo-tree.tree.scanMode" = "workspace";

          "emmet.includeLanguages".vue-html = "html";

          "eslint.run" = "onSave";
          "eslint.workingDirectories" = [{ mode = "auto" }];
          "eslint.validate" = [ "vue-html" ];

          "harper.dialect" = "British";

          # a bit hungry in the moment
          # "phpstan.enabled" = true;
          # yes scans the whole project, but once the cache is warm, it should be fast.
          # single file mode will skip and override the cache, not ideal.
          # "phpstan.singleFileMode" = false;
          "phpstan.memoryLimit" = "4G";

          "dbcode.connections" = [{
            "connectionId" = "${builtins.hashString "md5" config.env.DEVENV_ROOT}";
            "name" = "${builtins.baseNameOf config.env.DEVENV_ROOT}";
            "driver" = "mysql";
            "connectionType" = "host";
            "host" = "localhost";
            "port" = config.shopware.modules.mysql.port;
            "ssl" = false;
            "authMethod" = "password";
            "username" = "root";
            "savePassword" = "na";
            "readOnly" = false;
            "postConnectionSqlType" = "inline";
            "connectionTimeout" = 30;
            "idleTimeout" = 300;
            "maxConnections" = 10;
            "pinnedConnectionTimeout" = "0";
            "driverOptions" = {
                "retrievePublickey" = true;
                "disableMultiStatements" = false;
                "permitLocalInfile" = false;
            };
            "introspection"."progressive" = false;
          }];

          "phpunit.debuggerConfig" = "Listen for Xdebug"; # name of the launch config
        } // (lib.attrsets.optionalAttrs cfg.wrapper.phpunit {
          "phpunit.phpunit" = "${config.env.DEVENV_ROOT}/.devenv/profile/bin/vscode-phpunit-wrapper";
        });

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
          "elijah-potter.harper"
          "dbcode.dbcode" # db explorer
        ];

        extensions.unwantedRecommendations = [
          "vscode.typescript-language-features" # covered by "Vue.vscode-typescript-vue-plugin"
          "vscode.php-language-features" # covered by "bmewburn.vscode-intelephense-client"
        ];

        launch.version = "0.2.0";
        launch.configurations = [
          {
            name = "Listen for Xdebug";
            type = "php";
            request = "launch";
            port = config.shopware.modules.xdebug.port;
          }
        ];
      };
    in ''
      cd '${config.env.DEVENV_ROOT}'
      find ./custom/plugins -maxdepth 1 -type d ! -name 'plugins' -printf '%f\n' | sort | ${pkgs.jq}/bin/jq -Rn \
        --argjson ws '${builtins.toJSON base-config}' \
        --argjson excludedPlugins '${builtins.toJSON cfg.excludedPlugins}' \
        '$ws | .folders = .folders + [inputs | select(. as $i | $excludedPlugins | index($i) | not) | {path: ("custom/plugins/" + .)}]' > ${filename}
    '';

    packages = lib.lists.optional cfg.wrapper.phpunit (pkgs.writeScriptBin "vscode-phpunit-wrapper" ''
      #!/usr/bin/env php
      <?php

      $args = [...$argv];
      array_shift($args);

      function cmd_exists(string $cmd): bool
      {
        $out = "";
        $code = 1;
        exec(sprintf('/usr/bin/env bash -c "command -v %s"', $cmd), $out, $code);
        return $code === 0;
      }

      function exec_phpunit(string $bin): int {
        global $args;
        $code = 1;
        echo "wrapper: exec " . $bin . \PHP_EOL;
        passthru(sprintf('%s %s', $bin, implode(' ', array_map('escapeshellarg', $args))), $code);
        exit($code);
      }

      if (is_executable('vendor/bin/phpunit')) {
        exec_phpunit('vendor/bin/phpunit');
      } else if (is_executable('../../../vendor/bin/phpunit')) {
        exec_phpunit('../../../vendor/bin/phpunit');
      } else if (cmd_exists('phpunit')) {
        exec_phpunit('phpunit');
      } else {
        echo 'wrapper: no phpunit executable found' . \PHP_EOL;
        exit(1);
      }
    '');
  };
}