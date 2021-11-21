with import <nixpkgs> {};
derivation {
  name = "hello";
  system = builtins.currentSystem;
  builder = "${bash}/bin/bash";
  args = [ ./builder-hello.sh ];
  src = ./hello-2.10.tar.gz;
  inherit gnutar gzip gnumake gcc coreutils gawk gnused gnugrep;
  binutils = binutils-unwrapped;
}
