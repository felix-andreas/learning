{
  description = "Hello GTK 4";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-21.05";
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (
      system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
          {
            defaultPackage = pkgs.stdenv.mkDerivation
              {
                name = "hello-gtk4";
                nativeBuildInputs = with pkgs; [ pkg-config gtk4.dev ];
                builtInputs = with pkgs; [ gtk4 ];
                src = ./.;
              };
          }
    );
}
