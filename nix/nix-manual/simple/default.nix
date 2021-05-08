{ pkgs ? import <nixpkgs> {} }:

with pkgs; stdenv.mkDerivation {
    name = "simple";
    builder = ./builder.sh;
    src = ./simple.c;
    inherit gcc;
    inherit coreutils;
}
