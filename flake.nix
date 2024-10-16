{
  description = "Learning";

  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs.url = "unstable"; # local registry
    devshell.url = "github:numtide/devshell";
    nickel.url = "github:tweag/nickel";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-substituters = [ "https://tweag-nickel.cachix.org" ];
    extra-trusted-public-keys = [ "tweag-nickel.cachix.org-1:GIthuiK4LRgnW64ALYEoioVUQBWs0jexyoYVeLDBwRA=" ];
  };

  outputs = { self, nixpkgs, devshell, nickel, rust-overlay }:
    let
      allSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlays.default (import rust-overlay) ];
        };
        nickel = nickel.packages.${system};
      });
    in
    {
      devShells = forAllSystems ({ pkgs, nickel }: {
        nickel = pkgs.devshell.mkShell {
          motd = "";
          packages = with nickel; [
            nickel
            lsp-nls
            vscodeExtension
          ];
        };
        ml = pkgs.devshell.mkShell {
          motd = "";
          packages = with pkgs; [
            (python310.withPackages (p: with p; [ black isort ipython torch ]))
          ];
        };
        ocaml = pkgs.devshell.mkShell {
          motd = "";
          packages = with pkgs; [
            ocaml
            ocamlPackages.ocaml-lsp
            ocamlPackages.ocamlformat
            opam
            dune_3
            rlwrap
          ];
        };
        purescript = pkgs.devshell.mkShell {
          motd = "";
          packages = with pkgs; [
            spago
            purescript-language-server
          ];
        };
        r = pkgs.devshell.mkShell {
          motd = "";
          packages = with pkgs; [
            (rWrapper.override {
              packages = with pkgs.rPackages; [
                languageserver
                swirl
                tidyverse
                svglite
                # dplyr
                # ggplot2
              ];
            })
          ];
        };
        rust =
          let
            rust-toolchain = pkgs.rust-bin.selectLatestNightlyWith
              (toolchain: toolchain.default.override {
                extensions = [ "rust-src" "rust-analyzer" ];
              });
          in
          pkgs.devshell.mkShell {
            motd = "";
            packages = [
              rust-toolchain
              pkgs.cargo-expand
            ];
          };
        rust-ml =
          let
            rust-toolchain = pkgs.rust-bin.selectLatestNightlyWith
              (toolchain: toolchain.default.override {
                extensions = [ "rust-src" "rust-analyzer" ];
              });
          in
          pkgs.devshell.mkShell {
            motd = "";
            packages = [
              rust-toolchain
              pkgs.libtorch-bin.dev
              pkgs.pkg-config
            ];
            env = [
              {
                name = "LIBTORCH_LIB";
                value = with pkgs; libtorch-bin;
              }
              {
                name = "LIBTORCH_INCLUDE";
                value = with pkgs; libtorch-bin.dev;
              }
            ];
          };
        rust-wasm =
          let
            rust-toolchain = pkgs.rust-bin.selectLatestNightlyWith
              (toolchain: toolchain.default.override {
                extensions = [ "rust-src" "rust-analyzer" ];
                targets = [ "wasm32-unknown-unknown" ];
              });
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
      });
    };
}
