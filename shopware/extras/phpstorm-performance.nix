{ config, name, lib, ... }:
with lib; {
  options = {
    enable = mkEnableOption name;
    out = mkOption { type = types.attrs; default = {}; };
  };

  config.out = mkIf config.enable {
    file.".idea/devenv-web-module.xml".text = ''
    <?xml version="1.0" encoding="UTF-8"?>
    <module type="WEB_MODULE" version="4">
      <component name="NewModuleRootManager">
        <content url="file://$MODULE_DIR$">
          <excludeFolder url="file://$MODULE_DIR$/var/cache" />
          <excludeFolder url="file://$MODULE_DIR$/vendor-bin" />
          <excludeFolder url="file://$MODULE_DIR$/public" />
          <excludeFolder url="file://$MODULE_DIR$/src/Administration/Resources/administration/node_modules" />
          <excludeFolder url="file://$MODULE_DIR$/src/Administration/Resources/public" />
          <excludeFolder url="file://$MODULE_DIR$/src/Storefront/Resources/administration/node_modules" />
          <excludeFolder url="file://$MODULE_DIR$/src/Storefront/Resources/public" />
          <excludeFolder url="file://$MODULE_DIR$/src/Storefront/Resources/storefront/node_modules" />
          <excludePattern pattern="node_modules" />
          <excludePattern pattern="vendor" />
          <excludePattern pattern="public" />
        </content>
      </component>
    </module>
    '';

    file.".idea/devenv-project.xml".text = ''
    <?xml version="1.0" encoding="UTF-8"?>
    <project version="4">
      <component name="PhpInterpreters">
        <interpreters>
          <interpreter id="e3eceab1-06d2-4568-95e2-b3e691447e4a" name="Devenv" home="$PROJECT_DIR$/.devenv/profile/bin/php" auto="false" debugger_id="php.debugger.XDebug" />
        </interpreters>
      </component>
      <component name="PhpProjectSharedConfiguration" php_language_level="${config'.languages.php.version or "8.2"}">
        <option name="suggestChangeDefaultLanguageLevel" value="false" />
      </component>
    </project>
    '';


      # <component name="PHPCSFixerOptionsConfiguration">
      #   <option name="transferred" value="true" />
      # </component>
      # <component name="PhpCSFixer">
      #   <phpcsfixer_settings>
      #     <phpcs_fixer_by_interpreter interpreter_id="d87658de-6a6c-411e-875f-23302be9ed69" timeout="30000" />
      #     <PhpCSFixerConfiguration tool_path="$PROJECT_DIR$/vendor/bin/php-cs-fixer" />
      #     <phpcs_fixer_by_interpreter interpreter_id="e3eceab1-06d2-4568-95e2-b3e691447e4a" standards="DoctrineAnnotation;PER;PER-CS;PER-CS1.0;PER-CS2.0;PHP54Migration;PHP56Migration;PHP70Migration;PHP71Migration;PHP73Migration;PHP74Migration;PHP80Migration;PHP81Migration;PHP82Migration;PHP83Migration;PHP84Migration;PHPUnit100Migration;PHPUnit30Migration;PHPUnit32Migration;PHPUnit35Migration;PHPUnit43Migration;PHPUnit48Migration;PHPUnit50Migration;PHPUnit52Migration;PHPUnit54Migration;PHPUnit55Migration;PHPUnit56Migration;PHPUnit57Migration;PHPUnit60Migration;PHPUnit75Migration;PHPUnit84Migration;PHPUnit91Migration;PSR1;PSR12;PSR2;PhpCsFixer;Symfony" tool_path="/home/cyl3x/shopware/platform/vendor/bin/php-cs-fixer" timeout="30000" />
      #     <phpcs_fixer_by_interpreter asDefaultInterpreter="true" interpreter_id="e3eceab1-06d2-4568-95e2-b3e691447e4a" standards="DoctrineAnnotation;PER;PER-CS;PER-CS1.0;PER-CS2.0;PHP54Migration;PHP56Migration;PHP70Migration;PHP71Migration;PHP73Migration;PHP74Migration;PHP80Migration;PHP81Migration;PHP82Migration;PHP83Migration;PHP84Migration;PHPUnit100Migration;PHPUnit30Migration;PHPUnit32Migration;PHPUnit35Migration;PHPUnit43Migration;PHPUnit48Migration;PHPUnit50Migration;PHPUnit52Migration;PHPUnit54Migration;PHPUnit55Migration;PHPUnit56Migration;PHPUnit57Migration;PHPUnit60Migration;PHPUnit75Migration;PHPUnit84Migration;PHPUnit91Migration;PSR1;PSR12;PSR2;PhpCsFixer;Symfony" tool_path="/home/cyl3x/shopware/platform/vendor/bin/php-cs-fixer" timeout="30000" />
      #   </phpcsfixer_settings>
      # </component>
      # <component name="PhpExternalFormatter">
      #   <option name="externalFormatter" value="PHP_CS_FIXER" />
      # </component>
      # <component name="PhpStan">
      #   <PhpStan_settings>
      #     <PhpStanConfiguration tool_path="$PROJECT_DIR$/vendor/bin/phpstan" />
      #     <phpstan_by_interpreter interpreter_id="d87658de-6a6c-411e-875f-23302be9ed69" tool_path="/var/www/html/platform/vendor/bin/phpstan" timeout="60000" />
      #     <phpstan_by_interpreter asDefaultInterpreter="true" interpreter_id="e3eceab1-06d2-4568-95e2-b3e691447e4a" tool_path="$PROJECT_DIR$/vendor/bin/phpstan" timeout="60000" />
      #   </PhpStan_settings>
      # </component>
      # <component name="PhpStanOptionsConfiguration">
      #   <option name="config" value="$PROJECT_DIR$/phpstan.neon.dist" />
      #   <option name="transferred" value="true" />
      # </component>
  };
}
