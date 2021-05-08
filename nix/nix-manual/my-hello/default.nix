{ pkgs ? import <nixpkgs> {} }:

with pkgs; stdenv.mkDerivation {
    name = "my-hello";
    src = ./.;
    builder = ./builder.sh;
}
