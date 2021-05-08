{ pkgs ? import <nixpkgs> {} }:
with pkgs; python3Packages.buildPythonApplication {
    pname = "python-nix-gtk";
    src = ./.;
    version = "0.1";
    # nativeBuildInputs = [
    #     meson
    #     ninja
    #     gcc
    # ];
    # buildInputs = [
    #     gtk3
    #     gtk4
    # ];

    propagatedBuildInputs = [
        gtk4 gobject-introspection
        python3Packages.pygobject3
        python3Packages.numpy
    ];
}
