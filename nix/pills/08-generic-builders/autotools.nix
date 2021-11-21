pkgs: attrs: with pkgs;
derivation (
  {
    builder = "${bash}/bin/bash";
    args = [ ./builder-autotools.sh ];
    baseInputs = [ gnutar gzip gnumake gcc binutils-unwrapped coreutils gawk gnused gnugrep ];
    builtInputs = [];
    system = builtins.currentSystem;
  } // attrs
)
