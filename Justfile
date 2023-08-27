_list:
  just --list

build_app:
  (cd app && nix build)

build:
  just build_app
