{
  description = "Learning";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.devshell.url = "github:numtide/devshell";
  inputs.nickel.url = "github:tweag/nickel";

  nixConfig = {
    extra-substituters = [ "https://tweag-nickel.cachix.org" ];
    extra-trusted-public-keys = [ "tweag-nickel.cachix.org-1:GIthuiK4LRgnW64ALYEoioVUQBWs0jexyoYVeLDBwRA=" ];
  };

  outputs = { self, nixpkgs, flake-utils, devshell, nickel }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ devshell.overlay ]; };
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
        };
      });
}
