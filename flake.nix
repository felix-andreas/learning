{
  description = "Learning";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devshell.url = "github:numtide/devshell";
    nickel.url = "github:tweag/nickel";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  nixConfig = {
    extra-substituters = [ "https://tweag-nickel.cachix.org" ];
    extra-trusted-public-keys = [ "tweag-nickel.cachix.org-1:GIthuiK4LRgnW64ALYEoioVUQBWs0jexyoYVeLDBwRA=" ];
  };

  outputs = { self, nixpkgs, flake-utils, devshell, nickel, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlay (import rust-overlay) ];
        };
      in
      {
        devShells = {
          nickel = pkgs.devshell.mkShell {
            motd = "";
            packages = with nickel.packages.${system}; [
              nickel
              lsp-nls
              vscodeExtension
            ];
          };
          purescript = pkgs.devshell.mkShell {
            motd = "";
            packages = with pkgs; [
              spago
              purescript-language-server
            ];
          };
          rust-wasm =
            let
              rust-toolchain = pkgs.rust-bin.selectLatestNightlyWith
                (toolchain: toolchain.default.override {
                  extensions = [ "rust-src" "rust-analyzer" ];
                  targets = [ "wasm32-unknown-unknown" ];
                });
              cargo-leptos = pkgs.callPackage ./cargo-leptos.nix { };
            in
            pkgs.devshell.mkShell {
              motd = "";
              packages = with pkgs; [
                rust-toolchain
                cargo-leptos
                cargo-generate
                trunk
                sass
                wasm-pack
                binaryen
                pkg-config
                openssl.dev
                nodePackages.tailwindcss
              ];
              env = [
                {
                  name = "LD_LIBRARY_PATH";
                  value = with pkgs; lib.makeLibraryPath [ openssl ];
                }
                {
                  name = "PKG_CONFIG_PATH";
                  prefix = "$DEVSHELL_DIR/lib/pkgconfig";
                }
              ];
            };
        };
      });
}
