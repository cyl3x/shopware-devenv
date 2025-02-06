{ ... }:
{
  shopware = {
    enable = true;

    domain = "67.localhost"; # or null
    port = 3000;

    behaviour = {
      xdebug = true;
    };

    # exclusive with ssl.proxy
    ssl.standalone = {
      enable = true;
      fallback = 3080; # port + 80
    };

    # exclusive with ssl.standalone
    ssl.proxy = {
      enable = true;
      port = 2000;
      rootCA = "path-to-mkcert"; # or null to disable
    };

    modules = {
      admin-watcher = true;
      store-watcher = true;
      adminer = true;
      redis = true;
      mysql = true;
      mailpit = true;
      var-dump-server = true;
      rabbitmq = false;
      elasticsearch = false;
      cypress = false;
    };
  };

  shopwareProxy = {
    enable = true;

    port = 2000;

    platform = {
      "trunk.localhost" = 3000;
      "review.localhost" = 3100;
      "65.localhost" = 3200;
      "66.localhost" = 3300;
      "saas.localhost" = 3400;
    };

    app = {
      "swagtaxproviders.localhost" = 3000;
      "swagbraintree.localhost" = 8080;
      "swaginapppurchases.localhost" = 8001;
      "swagdhl.localhost" = 8090;
    };

    modules = {
      admin-watcher = true;
      store-watcher = true;
      adminer = true;
      mailpit = true;
      rabbitmq = false;
    };
  };

  shopware._lib.caddy = {
    adminer = { # subdomain
      http = true;
      https = true;
    };
  };
}