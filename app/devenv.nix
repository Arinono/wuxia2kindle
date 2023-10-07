{ pkgs, ... }:

{
  # See full reference at https://devenv.sh/reference/options/

  services.postgres = {
    enable = true;
    listen_addresses = "127.0.0.1";
    port = 5433;
    initialDatabases = [
      {
        name = "wuxia2kindle";
        schema = ./schema.sql;
      }
    ];
  };
}
