{ pkgs ? import <nixpkgs> {} }:
with pkgs; stdenv.mkDerivation {
    name = "meson-nix";
    src = ./.;
    nativeBuildInputs = [
        meson
        ninja
        gcc
    ];
    buildinputs = [
        gtk4
    ];
}
