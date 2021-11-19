with import <nixpkgs> {};
derivation {
  name = "simple";
  system = "x86_64-linux";
  builder = "${bash}/bin/bash";
  args = [ ./builder-simple.sh ];
  src = ./simple.c;
  inherit coreutils gcc;
}
