pkgs: attrs: with pkgs;
derivation (
  {
    builder = "${bash}/bin/bash";
    args = [ ./builder.sh ];
    setup = ./setup.sh;
    baseInputs = [ gnutar gzip gnumake gcc binutils-unwrapped coreutils gawk gnused gnugrep findutils patchelf ];
    builtInputs = [];
    system = builtins.currentSystem;
  } // attrs
)
