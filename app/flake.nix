{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          sqlxFilter = path: _type: builtins.match ".*json$" path != null;
          filterOrCargo = path: type: (sqlxFilter path type) || (craneLib.filterCargoSources path type);
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = filterOrCargo;
          };
          darwinNativeBuildInputs = with pkgs; [ rustToolchain pkg-config darwin.apple_sdk.frameworks.SystemConfiguration ];
          otherNativeBuildInputs = with pkgs; [ rustToolchain pkg-config ];
          nativeBuildInputs = if system == "aarch64-darwin" then darwinNativeBuildInputs else otherNativeBuildInputs;
          buildInputs = with pkgs; [ openssl ];
          commonArgs = {
            inherit src buildInputs nativeBuildInputs;
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          bin = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
          });
          dockerImage = pkgs.dockerTools.buildImage {
            name = "arinono/wuxia2kindle_app";
            tag = "latest";
            copyToRoot = [ bin ];
            config = {
              Cmd = [ "${bin}/bin/wuxia2kindle" ];
            };
          };
        in
        with pkgs;
        {
          packages =
            {
              inherit bin dockerImage;
              default = bin;
            };
          devShells.default = mkShell {
            inputsFrom = [ bin ];
          };
        }
      );
}
