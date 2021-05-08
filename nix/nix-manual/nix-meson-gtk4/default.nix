{ pkgs ? import <nixpkgs> {} }:
with pkgs; stdenv.mkDerivation {
    name = "meson-nix";
    src = ./.;
    nativeBuildInputs = [
        meson
        ninja
        pkg-config
    ];
    buildInputs = [
        gtk4
    ];
}
